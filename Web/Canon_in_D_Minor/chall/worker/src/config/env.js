const os = require('os');

const HOSTNAME = process.env.HOSTNAME || os.hostname();

module.exports = { HOSTNAME };
