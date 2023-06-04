import { defineConfig, presetWebFonts, presetWind, transformerDirectives } from 'unocss'

export default defineConfig({
  presets: [
    presetWind(),
    presetWebFonts({
      provider: 'google',
      fonts: { sans: 'Poppins' },
    }),
  ],
  transformers: [
    transformerDirectives({ enforce: 'pre' }),
  ],
  theme: {
    colors: {
      primary: '#FF9900',
    },
  },
})
