const cooldownCache = new Map();

function setCooldown(ticket) {
    cooldownCache.set(ticket, true);
}

function isInCooldown(ticket) {
    return cooldownCache.has(ticket);
}

module.exports = { setCooldown, isInCooldown };
