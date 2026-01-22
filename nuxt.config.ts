// https://nuxt.com/docs/api/configuration/nuxt-config
import dotenv from 'dotenv'

// Explicitly load env vars to ensure they are available for modules
dotenv.config()
dotenv.config({ path: '.env.local', override: true })

export default defineNuxtConfig({
  modules: [
    '@nuxt/eslint',
    '@nuxt/ui',
    '@nuxtjs/supabase'
  ],

  devtools: {
    enabled: true
  },

  app: {
    head: {
      link: [
        { rel: 'icon', type: 'image/png', href: '/p-symbol.png' }
      ]
    }
  },

  css: ['~/assets/css/main.css'],

  // Server-only runtime config (not exposed to client)
  // Keys without 'public' prefix are automatically server-only in Nuxt 3+
  runtimeConfig: {
    deepseekApiKey: process.env.DEEPSEEK_API_KEY,
    ingestApiKey: process.env.INGEST_API_KEY || '',
    public: {
      // Client-accessible config goes here (currently none needed)
    }
  },

  routeRules: {
    '/': { prerender: true },
    '/api/**': { redirect: undefined }  // Disable redirects for API routes
  },

  compatibilityDate: '2025-01-15',

  eslint: {
    config: {
      stylistic: {
        commaDangle: 'never',
        braceStyle: '1tbs'
      }
    }
  },

  supabase: {
    redirect: false,
    serviceKey: process.env.SUPABASE_SERVICE_KEY
  }
})
