
/* eslint-disable-next-line */
module.exports = {
  "env": {
    "node": true,
    "es2021": true
  },
  root: true,
  "extends": [
    "eslint:recommended",
    "plugin:@typescript-eslint/recommended",
    'plugin:prettier/recommended'
  ],
  "parser": "@typescript-eslint/parser",
  "parserOptions": {
    "ecmaVersion": 13,
    "sourceType": "module"
  },
  "plugins": [
    "@typescript-eslint", "prettier"
  ],
  "rules": {
    'no-unused-vars': 'warn',
    'prettier/prettier': 2,
  }
};
