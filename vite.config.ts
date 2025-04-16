/// <reference types="vitest" />
import { defineConfig } from 'vite'
import topLevelAwait from 'vite-plugin-top-level-await'
import wasm from 'vite-plugin-wasm'

export default defineConfig({
  plugins: [
    wasm(),
    topLevelAwait(),
  ],
  resolve: {
    mainFields: ['module'],
  },
  test: {
    exclude: ['src'],
    typecheck: {
      enabled: true,
    },
  },
})
