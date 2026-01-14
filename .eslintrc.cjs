module.exports = {
  root: true,
  extends: [
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended',
    'plugin:typescript-eslint/recommended',
    'prettier'
  ],
  plugins: ['@typescript-eslint', 'typescript-eslint'],
  parser: '@typescript-eslint/parser',
  parserOptions: {
    ecmaVersion: 2020,
    sourceType: 'module',
    extraFileExtensions: ['.svelte']
  },
  env: {
    browser: true,
    es2017: true,
    node: true
  },
  overrides: [
    {
      files: ['*.svelte'],
      parser: 'svelte-eslint-parser',
      parserOptions: {
        parser: '@typescript-eslint/parser'
      }
    },
    {
      files: ['**/*.ts', '**/*.tsx', '**/*.svelte'],
      rules: {
        '@typescript-eslint/no-unused-vars': [
          'error',
          {
            argsIgnorePattern: '^_',
            varsIgnorePattern: '^_',
            caughtErrorsIgnorePattern: '^_'
          }
        ],
        '@typescript-eslint/no-explicit-any': 'warn',
        '@typescript-eslint/explicit-function-return-type': 'off',
        '@typescript-eslint/explicit-module-boundary-types': 'off',
        '@typescript-eslint/no-non-null-assertion': 'warn'
      }
    }
  ],
  rules: {
    'no-console': 'warn',
    'no-debugger': 'warn',
    'prefer-const': 'error',
    'no-var': 'error'
  },
  ignorePatterns: [
    '*.config.js',
    '*.config.ts',
    'node_modules/',
    'build/',
    'dist/',
    '.svelte-kit/',
    'src-tauri/target/'
  ]
};
