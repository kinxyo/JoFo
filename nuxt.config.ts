export default defineNuxtConfig({
  devtools: { enabled: true },
  css: ['~/assets/css/main.css'],
  routeRules: {
    '/': { prerender: true }
  },
  nitro: {
    experimental: {
      wasm: true
    }
  },
  ssr: true,
  target: static,
  router: {
    base: '/notes/',
  },

})