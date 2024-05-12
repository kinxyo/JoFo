export default defineNuxtConfig({
  devtools: { enabled: false },
  css: ['~/assets/css/main.css'],
  routeRules: {
    '/': { prerender: true }
  },
  nitro: {
    experimental: {
      wasm: true
    }
  },
  ssr: false,
})