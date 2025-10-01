// Requires
const express = require('express')
const colors = require('colors')

// Settings
// Constants
// Variables
// Functions
// Router
const router = express.Router()

// Middleware

// Paths
router.get('/', (req, res)=>{ res.sendStatus(200) }) // API is OK

router.post('/login', (req, res)=>{

})

// Export
module.exports = router