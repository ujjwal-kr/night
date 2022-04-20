import axios from "axios"

const URL = "http://localhost:8000"

const Service = {
    getBlanance: async function() {
        return await axios.get(`${URL}/balance`).then(data => {
            return data.data
        }).catch(e => {
            return alert("Try running the rust server locally.")
        })
    },

    getTransactions: async function(page: Number) {
        return await axios.get(`${URL}/transactions?page=${page}`).then(data => {
            return data.data
        }).catch(e => {
            return e
        })
    },

    gamble: async function(amount: Number) {
        return await axios.post(`${URL}/gamble?amount=${amount}`).then(data => {
            return data.data
        }).catch(e => {
            return e
        })
    },

    countDataLength: async function() {
        return await axios.get(`${URL}/countmaster`).then(data => {
            return data.data.master_count
        }).catch(e => {
            return e
        })
    }
}

export default Service