module.exports = {
  "extends": [
    "airbnb-typescript",
    "plugin:@typescript-eslint/recommended",
    "plugin:@typescript-eslint/recommended-requiring-type-checking"
  ],
  parserOptions: {
    project: './tsconfig.json',
  },
  rules:{
    "no-console": "error",
    "react/prop-types": "off",
    "react/jsx-props-no-spreading": "off",
    "@typescript-eslint/unbound-method": "off",
    "max-len": ["error", 160]
  }
};