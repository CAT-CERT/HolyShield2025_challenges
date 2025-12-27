const { COOKIE_NAME } = require('../config/constants');

function extractTicketFromCookie(req) {
    const cookieHeader = req.headers.cookie || '';
    const match = cookieHeader.match(new RegExp(`(?:^|;\\s*)${COOKIE_NAME}=([^;]+)`));
    return match ? match[1] : null;
}

function extractTicketFromBody(req) {
    if (req.body && typeof req.body.ticket === 'string') {
        return req.body.ticket;
    }
    return null;
}

function extractTicket(req) {
    return extractTicketFromCookie(req) || extractTicketFromBody(req);
}

module.exports = { extractTicket, extractTicketFromCookie, extractTicketFromBody };
