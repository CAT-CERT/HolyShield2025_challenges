const express = require('express');
const { health } = require('../controllers/healthController');
const { extractTicketFromCookie, extractTicketFromBody } = require('../middleware/tickets');
const { isValidTicket } = require('../services/tickets');
const { isInCooldown } = require('../services/cooldown');
const { ORCHESTRATOR_URL } = require('../config/constants');

const router = express.Router();

router.get('/api/health', health);

router.post('/diagnostics', async (req, res) => {
    const cueRelay = req.header('x-cue-relay') === '1';
    const cookieTicket = extractTicketFromCookie(req);
    const bodyTicket = extractTicketFromBody(req);
    const ticket = cueRelay ? bodyTicket : cookieTicket;
    if (!ticket) {
        return res.status(403).json({ error: 'ticket required' });
    }
    if (!(await isValidTicket(ticket))) {
        return res.status(403).json({ error: 'invalid ticket' });
    }
    if (isInCooldown(ticket)) {
        return res.status(403).json({ error: 'admin cooldown active' });
    }

    const targetUrl = req.body && typeof req.body.url === 'string' ? req.body.url : '';
    if (!targetUrl) {
        return res.status(400).json({ error: 'missing url' });
    }

    try {
        const response = await fetch(ORCHESTRATOR_URL, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ url: targetUrl }),
        });
        const responseBody = await response.text();
        res.status(response.status);
        res.setHeader('Content-Type', response.headers.get('content-type') || 'text/plain');
        return res.send(responseBody);
    } catch (err) {
        return res.status(500).json({ error: 'diagnostics failed' });
    }
});

module.exports = router;
