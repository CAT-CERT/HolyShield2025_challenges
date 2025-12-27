const TICKET_TTL_SEC = 300;
const COOKIE_NAME = 'ticket';
const ALLOWED_CUE_HOST = 'nginx';
const ORCHESTRATOR_URL = 'http://orchestrator:8080/admin/health-check';
const CUE_TIMEOUT_MS = 5000;

module.exports = {
    TICKET_TTL_SEC,
    COOKIE_NAME,
    ALLOWED_CUE_HOST,
    ORCHESTRATOR_URL,
    CUE_TIMEOUT_MS,
};
