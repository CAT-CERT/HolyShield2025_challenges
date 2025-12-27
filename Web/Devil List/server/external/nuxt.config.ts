// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    devtools: { enabled: true },
    nitro: {
        preset: 'node-server'
    },
    runtimeConfig: {
        internalHost: process.env.INTERNAL_HOST,
        internalPort: process.env.INTERNAL_PORT,
        adminHost: process.env.ADMIN_HOST,
        adminPort: process.env.ADMIN_PORT,
        public: {}
    }
})
