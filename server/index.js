//**
// 
//  Archiva Server
//  Griffin Dalby
//
//  Controls the backend of the archiva system, handling user input &
//  serving data back.
//
// */

// Requires
const fs = require('fs')
const path = require('path')
const yaml = require('js-yaml')

const express = require('express')

const colors = require('colors')
const prompt = require('prompt-sync')()

const arcFs = require('./archiva.fs')
const global = require('./globalCache')

// Routes
// const route_identity = require('./routes/identity')
const route_files = require('./routes/files')

// Settings
const purpleShades = [
    "#BA55D3",
    "#9127bbff",
    "#7f2fa3ff",
    "#7941b4ff",
    "#7a39b8ff",
    "#782cc5ff",
];

// Constants
// Variables
// Functions
function colorText(text, hex) {
  const r = parseInt(hex.slice(1, 3), 16);
  const g = parseInt(hex.slice(3, 5), 16);
  const b = parseInt(hex.slice(5, 7), 16);
  return `\x1b[38;2;${r};${g};${b}m${text}\x1b[0m`;
}

// Setup Environment
console.clear()
console.log(``)
console.log(colorText(`      _                      __        _                 `, purpleShades[0]))
console.log(colorText(`     / \\                    [  |      (_)                `, purpleShades[1]))
console.log(colorText(`    / _ \\     _ .--.  .---.  | |--.   __  _   __  ,--.   `, purpleShades[2]))
console.log(colorText(`   / ___ \\   [ \`/'\`\\]/ /'\`\\] | .-. | [  |[ \\ [  ]\`'_\\ :  `, purpleShades[3]))
console.log(colorText(` _/ /   \\ \\_  | |    | \\__.  | | | |  | | \\ \\/ / // | |, `, purpleShades[4]))
console.log(colorText(`|____| |____|[___]   '.___.'[___]|__][___] \\__/  \\'-;__/ `, purpleShades[5]))
console.log(``)
console.log(`${colorText(`Made with`, purpleShades[3])} ${colors.red('♥')} ${colorText('by Griffin Dalby', purpleShades[3])}\n`)
console.log(`[${colors.magenta('Archiva')}] Setting up environment.`)

if (process.cwd()==__dirname) {
    console.log(`[${colors.red('Archiva')}] Archiva shouldn't be ran from the source directory!`)
    console.log(`[${colors.red('Archiva')}] Go a directory above and run ${colors.bold('run_server.bat')} for a proper start.`)
    process.exit(0) }
arcFs.validate_environment()

let config = yaml.load(fs.readFileSync(arcFs.config_path))
if (!config) {
    console.log(`[${colors.red('Archiva')}] ${colors.red('Failed to load config.yml! Defaulting to default_config')}`)

    config = {}
    Object.assign(config, arcFs.default_config)
} else {
    console.log(`[${colors.magenta('Archiva')}] Loaded config.yml!`)
    
    // Validate EULA acceptance
    if (!config.eulaAcceptance) {
        console.log(`[${colors.red('Archiva')}] Hold on! Before you can use Archiva, ${colors.underline('you must agree to Archiva EULA!')}`)
        console.log(`${colors.bold(colors.red('1.'))} Read through ${colors.underline('archiva.eula')} included with this project`)
        console.log(`${colors.bold(colors.red('2.'))} Open ${colors.underline(arcFs.config_path)} and set ${colors.cyan('eulaAcceptance')}: ${colors.green('true')}`)
        console.log(`${colors.bold(colors.red('3.'))} Restart Archiva once you're done!`)
        
        process.exit(0)
    }

    // Validate storage path
    if (!fs.existsSync(config.storage.path)) {
        fs.mkdirSync(config.storage.path)
        console.log(`[${colors.magenta('Archiva')}] Generated storage folder @ ${colors.bold(config.storage.path)}`)
    }
}

global.setValue('config', config)

// Start Server
const server = express()
server.use(express.json())
server.use(express.urlencoded({ extended: true }))
server.use(express.static(path.join(__dirname, '../client')))

const req_intercept = (req, res, next) => {
    config.logging.verbosity>=3 ? console.log(`[${colors.cyan('HTTPS')}] ${colors.bold(req.originalUrl)} was ${colors.bold(req.method)}(ed)`) : null
    next()
}
server.use(req_intercept)

server.get('/', (req, res) => {
    res.sendFile(path.join(__dirname, '../client', 'index.html'))
})

server.listen(config.server.port, () => {
    console.log(`[${colors.cyan('HTTPS')}] Server established! (@ localhost:${colors.underline(config.server.port)})`)
})