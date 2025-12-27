const nginxConfigPath = '/etc/nginx/conf.d/dynamic_upstream.conf';
const HEALTH_CHECK_INTERVAL = 15000;
const EXPOSED_WORKERS = ['worker-a', 'worker-b'];

module.exports = {
    nginxConfigPath,
    HEALTH_CHECK_INTERVAL,
    EXPOSED_WORKERS,
};
