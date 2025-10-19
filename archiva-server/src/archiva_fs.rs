//**
// 
//  Archiva File System
//  Griffin Dalby
//
//  File system wrapper built specifically to aid efficiency and control.
//
// */

// Modules
use std::{fs::{self, File}, io::Write, path::Path};
use serde::{Serialize, Deserialize};
use serde_yaml;
use colored::*;

// Settings
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    eula_acceptance: bool,
    server: ServerConfig,
    storage: StorageConfig,
    compression: CompressionConfig,
    logging: LoggingConfig,
}

#[derive(Debug, Deserialize, Serialize)]
struct ServerConfig {
    port: u16,
    host: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct StorageConfig {
    path: String,
    allocated_mb: u32
}

#[derive(Debug, Deserialize, Serialize)]
struct CompressionConfig {
    automatic: bool,
    format: String,
    level: u8
}

#[derive(Debug, Deserialize, Serialize)]
struct LoggingConfig {
    level: String,
    verbosity: u8,
    log_path: String
}

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
    let config_path = Path::new("config.yml");
    if !config_path.exists() {
        let comment = r#"
# =================================================
# Archiva Configuration File
# Generated Dynamically - documentation available on github
# Edit carefully - changes take effect on restart.
# =================================================

"#;
        let yaml_str = serde_yaml::to_string(&default_config())
            .expect("Failed to serialize default config!");
        let combo_str = format!("{}{}", comment, yaml_str);
        let mut file: File = File::create("config.yml").expect("Failed to create config.yml!");
        file.write_all(combo_str.as_bytes()).expect("Failed to write config.yml!");
        
        println!("[{}] Generated config @ {}", "Archiva".magenta(), config_path.display());
    }
}