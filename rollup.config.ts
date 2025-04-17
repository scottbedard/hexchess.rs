import { defineConfig } from 'rollup'
import { minify } from 'rollup-plugin-esbuild-minify'
import typescript from '@rollup/plugin-typescript'

export default defineConfig([
  {
    input: 'src/index.ts',
    output: [
      {
        file: 'dist/index.mjs',
        format: 'es',
      },
      {
        file: 'dist/index.bundle.js',
        format: 'iife',
        name: 'Hexchess',
        plugins: [
          minify(),
        ],
        sourcemap: true,
      },
    ],
    plugins: [
      typescript(),
    ],
  },
])
