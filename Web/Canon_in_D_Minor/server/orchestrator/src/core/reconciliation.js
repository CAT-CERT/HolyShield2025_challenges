const { checkWorkerHealth } = require('../services/health');
const { updateNginxConfig } = require('../services/nginx');
const { EXPOSED_WORKERS } = require('../config/constants');

async function reconciliationLoop(healthyWorkers, nginxConfigPath) {
    console.log("Running reconciliation loop (Health Check)...");

    const baseSet = EXPOSED_WORKERS.map(name => ({
        name,
        healthUrl: `http://${name}:8080/admin/api/health`,
    }));
    const healthy = await checkWorkerHealth(baseSet);

    healthyWorkers.length = 0;
    healthyWorkers.push(...healthy);

    await updateNginxConfig(healthyWorkers, nginxConfigPath);
}

module.exports = { reconciliationLoop };
