//**
// 
//  Archiva File System
//  Griffin Dalby
//
//  File system wrapper built specifically to aid efficiency and control.
//
// */

// Requires
const fs = require('fs');
const yaml = require('js-yaml');

const colors = require('colors')

const global = require('./globalCache')

// Settings
const config_path = './config.yml'
const default_config = {
    server: {
        port: 4514,
        host: 'localhost',
    },
    storage: {
        path: './storage',
        allocatedMB: 25000,
    },
    compression: {
        automatic: true,
        format: 'zip',
        level: 6,
    },
    logging: {
        level: 'info',
        verbosity: 3,
        logPath: './'
    },
    
    eulaAcceptance: false,
}

const eula = `
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
`

// Constants
// Variables
let config

// Functions
function validate_environment() {
    // Validate config.yml
    if (!fs.existsSync(config_path)) {
        const yaml_str = yaml.dump(default_config, { noRefs: true, indent: 2 })
        const comment = `
# =================================================
# Archiva Configuration File
# Generated Dynamically - documentation available on github
# Edit carefully - changes take effect on restart.
# =================================================

`;
    
        fs.writeFileSync(config_path, comment+yaml_str, 'utf8')
        console.log(`[${colors.magenta('Archiva')}] Generated config @ ${colors.bold(config_path)}`)
    }

    // Validate eula.txt
    if (!fs.existsSync('./eula.txt')) {
        const now = new Date()
        const comment = `
# =================================================
# Archiva End User License Agreement (EULA)
# Generated alongside config.yml @ ${now.toString()}
# =================================================

`
        fs.writeFileSync('./eula.txt', eula)
        console.log(`[${colors.magenta('Archiva')}] Generated eula @ ${colors.bold('./eula.txt')}`)
    }
}

// Module
module.exports.config_path = config_path
module.exports.default_config = default_config

module.exports.validate_environment = validate_environment