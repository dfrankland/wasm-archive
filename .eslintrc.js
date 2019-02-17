const typescriptEslintRecommended = require('@typescript-eslint/eslint-plugin/lib/configs/recommended.json');
const typescriptEslintPrettier = require('eslint-config-prettier/@typescript-eslint');

module.exports = {
  env: {
    node: true,
  },
  parser: 'babel-eslint',
  extends: ['airbnb-base', 'plugin:@typescript-eslint/recommended','prettier', 'prettier/@typescript-eslint'],
  plugins: ['prettier'],
  rules: {
    'prettier/prettier': 'error',
    'import/no-extraneous-dependencies': [
      'error',
      {
        devDependencies: [
          'rollup.config.js',
          '.eslintrc.js',
        ],
      },
    ],
  },
  overrides: [
    {
      files: ['*.ts'],
      parser: '@typescript-eslint/parser',
      // NOTE: Workaround for no nested extends possible.
      // See https://github.com/eslint/eslint/issues/8813.
      // Working solution would be following, if we had nested extends:
      // ```
      // extends: [
      //   'airbnb-base',
      //   'plugin:@typescript-eslint/recommended',
      //   'prettier/@typescript-eslint',
      //   'prettier',
      // ],
      // ```
      plugins: ['@typescript-eslint', 'prettier'],
      rules: Object.assign(typescriptEslintRecommended.rules, typescriptEslintPrettier.rules),
      settings: {
        'import/resolver': {
          node: {
            extensions: ['.js', '.ts'],
          },
        },
      },
    },
    {
      files: ['*.test.ts'],
      env: {
        jest: true,
      },
    },
  ],
};
