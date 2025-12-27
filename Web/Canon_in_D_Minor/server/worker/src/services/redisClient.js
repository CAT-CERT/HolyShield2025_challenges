const { createClient } = require('redis');

const redisUrl = process.env.REDIS_URL || 'redis://redis:6379';
const redis = createClient({ url: redisUrl });

redis.on('error', (err) => {
    console.error('Redis error:', err);
});

redis.connect().catch((err) => {
    console.error('Failed to connect to Redis:', err);
});

module.exports = { redis };
