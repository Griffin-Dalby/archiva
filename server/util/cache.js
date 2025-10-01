// Requires
// Settings
// Constants
// Variables
// Functions
// Cache

class Cache {
    constructor() {
        this.data = {}
    }

    setValue(key, value) {
        this.data[key] = value
    }

    getValue(key) {
        return this.data[key]
    }

    hasKey(key) {
        return key in this.data
    }
    
    deleteKey(key) {
        delete this.data[key]
    }

    clear() {
        this.data = {}
    }

    keys() {
        return Object.keys(this.data)
    }
}

module.exports = Cache