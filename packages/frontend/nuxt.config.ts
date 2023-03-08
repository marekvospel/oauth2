// https://nuxt.com/docs/api/configuration/nuxt-config
import { i18n } from './config/i18n'

export default defineNuxtConfig({
  modules: [
    '@unocss/nuxt',
    '@nuxtjs/i18n',
    '@nuxt/devtools'
  ],
  css: [
    '@unocss/reset/tailwind.css'
  ],
  hooks: {
    'vite:extendConfig': (config, { isServer }) => {
      if (isServer) {

        const noExternal = config.ssr!.noExternal as string[]
        noExternal.push('vue-i18n')
      }
    }
  },
  i18n
})
