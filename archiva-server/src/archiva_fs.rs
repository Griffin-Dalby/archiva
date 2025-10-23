//**
// 
//  Archiva File System
//  Griffin Dalby
//
//  File system wrapper built specifically to aid efficiency and control.
//
// */

// Modules
use std::{fs::File, io::Write, path::Path, time::SystemTime};
use serde::{Serialize, Deserialize};
use serde_yaml;
use colored::*;
use chrono::{offset::Local, DateTime};

// Macros
macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
        pub(crate) struct $name {
            $(pub $field: $t),*
        }
    };
}

// Settings
pub_struct!(Config {
    server: ServerConfig,
    storage: StorageConfig,
    compression: CompressionConfig,
    logging: LoggingConfig,
    eula_acceptance: bool,
});

pub_struct!(ServerConfig {
    port: u16,
    host: String,
});

pub_struct!(StorageConfig {
    path: String,
    allocated_mb: u32,
});

pub_struct!(CompressionConfig {
    automatic: bool,
    format: String,
    level: u8,
});

pub_struct!(LoggingConfig {
    level: String,
    verbosity: u8,
    log_path: String,
});

// Constants
// Variables
// Functions
// Archiva.fs
pub fn default_config() -> Config {
    return Config {
        eula_acceptance: false,
        server: ServerConfig { port: 4514, host: "localhost".to_string() },
        storage: StorageConfig { path: "./storage".to_string(), allocated_mb: 25000 },
        compression: CompressionConfig { automatic: true, format: "zip".to_string(), level: 6 },
        logging: LoggingConfig { level: "info".to_string(), verbosity: 3, log_path: "./".to_string() }
    };
}

pub const EULA: &str = r#"
License & Terms of Use
----------------------
This software, Archiva, is licensed under the Apache License, Version 2.0. 
By using Archiva, you agree to the following:

1. License
----------
- Archiva is Apache-2.0 licensed; you may use, modify, and distribute this software freely.
- However, redistributions MUST include this EULA, and the Apache-2.0 notice.

2. User Responsibility
----------------------
- The user is solely responsible for their data and backups
- Archiva developers are NOT liable for data loss, corruption, or any damages arising from use.
- The user may not use Archiva to store illegal, harmful, or copyrighted material without permission.

4. Contributions & Modifictions
-------------------------------
- Contributions to Archiva are governed by the Apache-2.0 license.
- Modifications made may be redistributed under Apache-2.0, only with proper attribution.

5. Disclaimer
-------------
- Archiva is provided "AS IS", without warranty of any kind.
- The author explicitly disclaims all implied warrienties, including merchantability or fitness for a particular purpose.

6. Termination
--------------
- This EULA terminates automatically if you violate any terms above.
- Upon termination, The user must cease use of Archiva, and delete all copies from your systems.

7. Contact & Notices
--------------------
- For questions, bug reports, or contributions, contact: griffindalby7@outlook.com
"#;

// Validation
pub fn validate_environment() {
    let config_path: &Path = Path::new("config.yml");
    if !config_path.exists() {
        let comment: &'static str = r#"
# =================================================
# Archiva Configuration File
# Generated Dynamically - documentation available on github
# Edit carefully - changes take effect on restart.
# =================================================

"#;
        let yaml_str: String = serde_yaml::to_string(&default_config())
            .expect("Failed to serialize default config!");
        let combo_str: String = format!("{}{}", comment, yaml_str);
        let mut file: File = File::create("config.yml").expect("Failed to create config.yml!");
        file.write_all(combo_str.as_bytes()).expect("Failed to write config.yml!");
        
        println!("[{}] Generated config @ {}", "Archiva".magenta(), config_path.display());
    }

    let eula_path: &Path = Path::new("eula.txt");
    if !eula_path.exists() {
        let now = SystemTime::now();
        let datetime: DateTime<Local> = now.into();
        let comment = format!(r#"
# =================================================
# Archiva End User License Agreement (EULA)
# Generated alongside config.yml @ {}
# =================================================
"#, datetime.format("%d/%m/%Y %T"));

        let combo_str = format!("{}{}", comment, EULA);
        let mut file: File = File::create("eula.txt").expect("Failed to create eula.txt!");
        file.write_all(combo_str.as_bytes()).expect("Failed to write to eula.txt!");

        println!("[{}] Generated eula @ {}", "Archiva".magenta(), eula_path.display());
    }
}