import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],
  base: 'https://ikerurle.github.io/larva/',
  server: {
    https: {
      cert : './cert.pem',
      key: './cert-key.pem',
    }
  }
})
