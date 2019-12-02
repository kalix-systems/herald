module.exports = {
  parser: "@typescript-eslint/parser",
  plugins: ["@typescript-eslint"],
  extends: ["plugin:@typescript-eslint/recommended"]
  rules: {
    "camelCase": [ {"allow": "Key_.*"}]
  }
};
