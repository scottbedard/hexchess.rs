import eslint from '@eslint/js'
import globals from 'globals'
import stylistic from '@stylistic/eslint-plugin'
import tseslint from 'typescript-eslint'

export default tseslint.config(
  {
    ignores: [
      'dist',
      'node_modules',
      'pkg',
      'target',
    ],
  },

  // eslint rules
  eslint.configs.recommended,
  tseslint.configs.recommended,
  {
    rules: {
      'no-sparse-arrays': 'off',
    },
  },

  // stylistic rules
  {
    languageOptions: {
      globals: {
        ...globals.node,
      },
      parserOptions: {
        parser: tseslint.parser,
      },
    },
    plugins: {
      '@stylistic': stylistic,
    },
    rules: {
      '@stylistic/arrow-parens': ['error', 'as-needed'],
      '@stylistic/brace-style': ['error', '1tbs'],
      '@stylistic/comma-dangle': ['error', {
        arrays: 'always-multiline',
        exports: 'never',
        functions: 'never',
        imports: 'never',
        objects: 'always-multiline',
      }],
      '@stylistic/eol-last': ['error', 'always'],
      '@stylistic/indent': ['error', 2],
      '@stylistic/indent-binary-ops': ['error', 2],
      '@stylistic/no-multi-spaces': 'error',
      '@stylistic/no-multiple-empty-lines': ['error', { max: 1 }],
      '@stylistic/no-trailing-spaces': 'error',
      '@stylistic/padded-blocks': ['error', 'never'],
      '@stylistic/quotes': ['error', 'single'],
      '@stylistic/semi': ['error', 'never'],
    },
  },

  {
    files: ['src/bundle.mjs'],
    rules: {
      '@stylistic/quotes': ['error', 'double'],
      '@stylistic/semi': ['error', 'always'],
    },
  },

  {
    files: ['src/wasm/prepend.d.ts'],
    rules: {
      '@typescript-eslint/no-unused-vars': 'off',
    },
  }
)
