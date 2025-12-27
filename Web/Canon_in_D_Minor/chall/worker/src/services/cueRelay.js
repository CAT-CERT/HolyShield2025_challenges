const { ALLOWED_CUE_HOST, CUE_TIMEOUT_MS } = require('../config/constants');

function parseCueTarget(targetUrl) {
    let parsed;
    try {
        parsed = new URL(targetUrl);
    } catch {
        return null;
    }

    if (parsed.protocol !== 'http:' || parsed.hostname !== ALLOWED_CUE_HOST) {
        return null;
    }

    return parsed;
}

async function sendCue(targetUrl, headers, method, body) {
    const controller = new AbortController();
    const timeout = setTimeout(() => controller.abort(), CUE_TIMEOUT_MS);
    try {
        const response = await fetch(targetUrl, {
            method,
            headers,
            body: body ? JSON.stringify(body) : undefined,
            signal: controller.signal,
        });
        const payload = await response.text();
        return {
            status: response.status,
            contentType: response.headers.get('content-type') || 'text/plain',
            body: payload,
        };
    } finally {
        clearTimeout(timeout);
    }
}

module.exports = { parseCueTarget, sendCue };
