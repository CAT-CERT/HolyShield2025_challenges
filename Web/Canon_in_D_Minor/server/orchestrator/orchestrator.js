const express = require('express');
const { healthyWorkers } = require('./src/config/initialState');
const { reconciliationLoop } = require('./src/core/reconciliation');
const {
    nginxConfigPath,
    HEALTH_CHECK_INTERVAL,
} = require('./src/config/constants');

const app = express();
const port = 8080;
const FLAG = process.env.FLAG || '[**REDACTED**]';

app.use(express.json());

app.get('/health', (req, res) => {
    res.send("ok");
});

app.post('/admin/health-check', async (req, res) => {
    const targetUrl = req.body && typeof req.body.url === 'string' ? req.body.url : '';
    if (!targetUrl) {
        return res.status(400).json({ error: 'missing url' });
    }
    try {
        const controller = new AbortController();
        const timeout = setTimeout(() => controller.abort(), 5000);
        await fetch(targetUrl, { signal: controller.signal });
        clearTimeout(timeout);
        res.status(204).end();
    } catch (err) {
        res.status(502).send(FLAG);
    }
});

async function startReconciliationLoop() {
    try {
        await reconciliationLoop(
            healthyWorkers,
            nginxConfigPath
        );
    } catch (e) {
        console.error(`Reconciliation loop failed: ${e.message || e}`);
    } finally {
        setTimeout(startReconciliationLoop, HEALTH_CHECK_INTERVAL);
    }
}

app.listen(port, async () => {
    console.log(`Orchestrator listening on port ${port}`);
    startReconciliationLoop();
});
