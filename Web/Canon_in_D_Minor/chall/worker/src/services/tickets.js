const crypto = require('crypto');
const { redis } = require('./redisClient');
const { TICKET_TTL_SEC } = require('../config/constants');

function generateTicket() {
    return crypto.randomBytes(16).toString('hex');
}

async function storeTicket(ticket) {
    await redis.setEx(`ticket:${ticket}`, TICKET_TTL_SEC, '1');
}

async function isValidTicket(ticket) {
    if (!ticket) {
        return false;
    }
    const exists = await redis.exists(`ticket:${ticket}`);
    return exists === 1;
}

module.exports = { generateTicket, storeTicket, isValidTicket };
