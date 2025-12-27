const Docker = require('dockerode');
const fs = require('fs');

const docker = new Docker({ socketPath: '/var/run/docker.sock' });
let lastConfig = null;

async function updateNginxConfig(healthyWorkers, nginxConfigPath) {
    
    const upstreamEntries = healthyWorkers
        .map(w => `    server ${w.name}:8080 max_fails=3 fail_timeout=5s;`)
        .join('\n');
    
    const newConfig = `upstream worker_vms {\n    hash $cookie_ticket consistent;\n${upstreamEntries}\n}`;

    if (lastConfig === newConfig) {
        return;
    }

    try {
        fs.writeFileSync(nginxConfigPath, newConfig);

        const nginxContainer = docker.getContainer('nginx');
        const execCmd = await nginxContainer.exec({
            Cmd: ['nginx', '-s', 'reload'],
            AttachStdout: true,
            AttachStderr: true
        });
        await execCmd.start();
        
        const names = healthyWorkers.map(w => w.name).join(', ');
        console.log(`Nginx reloaded. Load balancing workers: [${names}]`);
        lastConfig = newConfig;

    } catch (e) {
        console.error(`Failed to update Nginx: ${e.message}`);
    }
}

module.exports = { updateNginxConfig };
