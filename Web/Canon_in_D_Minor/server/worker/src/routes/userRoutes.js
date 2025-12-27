const express = require('express');
const { extractTicket } = require('../middleware/tickets');
const { isValidTicket } = require('../services/tickets');
const { parseCueTarget, sendCue } = require('../services/cueRelay');

const router = express.Router();

router.post('/cue-test', async (req, res) => {
    const ticket = extractTicket(req);
    if (!(await isValidTicket(ticket))) {
        return res.status(403).json({ error: 'invalid ticket' });
    }

    const targetUrl = req.body && typeof req.body.url === 'string' ? req.body.url : '';
    const method = req.body && typeof req.body.method === 'string'
        ? req.body.method.toUpperCase()
        : 'GET';
    const payload = req.body && typeof req.body.body === 'object' && req.body.body
        ? req.body.body
        : null;
    const parsed = parseCueTarget(targetUrl);
    if (!parsed) {
        return res.status(403).json({ error: 'cue target not allowed' });
    }

    const headers = req.body && typeof req.body.headers === 'object' && req.body.headers
        ? req.body.headers
        : {};
    if (payload && !headers['Content-Type']) {
        headers['Content-Type'] = 'application/json';
    }

    try {
        const result = await sendCue(parsed.toString(), headers, method, payload);
        res.status(result.status);
        res.setHeader('Content-Type', result.contentType);
        return res.send(result.body);
    } catch (err) {
        return res.status(500).json({ error: 'cue relay failed' });
    }
});

module.exports = router;
