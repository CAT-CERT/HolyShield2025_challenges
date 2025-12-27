const fs = require('fs');
const path = require('path');

const LOG_DIR = path.join(__dirname, '..', '..', 'logs');
const ERROR_LOG_PATH = path.join(LOG_DIR, 'error.log');

if (!fs.existsSync(LOG_DIR)) {
    fs.mkdirSync(LOG_DIR, { recursive: true });
}

function appendHealthErrorLog(workerName) {
    const line = `[${new Date().toISOString()}] ${workerName}: error\n`;
    fs.appendFile(ERROR_LOG_PATH, line, (err) => {
        if (err) {
            console.error('Failed to write error log', err);
        }
    });
}

async function checkWorkerHealth(healthyWorkers) {
    const result = [];

    for (const worker of healthyWorkers) {
        try {
            const controller = new AbortController();
            const timeout = setTimeout(() => controller.abort(), 5000);
            
            const response = await fetch(worker.healthUrl, { signal: controller.signal });
            clearTimeout(timeout);
            
            if (response.ok) {
                result.push(worker);
            } else {
                console.error(`Worker ${worker.name} reported unhealthy status.`);
            }
        } catch {
            console.error(`Health check failed for ${worker.name}. Assuming unhealthy.`);
            appendHealthErrorLog(worker.name);
        }
    }

    return result;
}

module.exports = { checkWorkerHealth };

