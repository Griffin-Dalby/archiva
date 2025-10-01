// Requires
const express = require('express')
const colors = require('colors')

const arcFs = require('../archiva.fs')
const global = require('../globalCache')

// Settings
// Constants
// Variables
// Functions
// Router
const router = express.Router()

// Middleware

// Paths
router.get('/', (req, res)=>{ res.sendStatus(200) }) // API is OK

router.post('/upload', (req, res)=>{
    
})

router.get('/exists', (req, res)=>{
    const config = global.getValue('config')
    
})
router.get('/list', (req, res)=>{

})
router.get('/download', (req, res)=>{

})

// Export
module.exports = router