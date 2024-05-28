/** @type { import("eslint").Linter.Config } */
module.exports = {
    root: true,
    extends: [
        'eslint:recommended',
        'plugin:@typescript-eslint/recommended',
        'plugin:svelte/recommended',
        'prettier'
    ],
    parser: '@typescript-eslint/parser',
    plugins: ['@typescript-eslint'],
    parserOptions: {
        sourceType: 'module',
        ecmaVersion: 2020,
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
        }
    ],
    rules: {
        'brace-style': ['error', '1tbs'],
        'no-trailing-spaces': 'error',
        'quote-props': ['error', 'as-needed'],
        curly: ['error', 'multi-line'],
        quotes: ['error', 'single'],
        semi: ['error', 'never'],
        indent: ['error', 4, { SwitchCase: 1, ignoredNodes: ['PropertyDefinition'] }]
    }
}
