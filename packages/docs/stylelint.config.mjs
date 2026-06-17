/** @type {import('stylelint').Config} */
export default {
  extends: ["stylelint-config-standard"],
  plugins: ["stylelint-value-no-unknown-custom-properties"],
  rules: {
    // Catch typos in the elmethis design tokens and our own custom properties.
    "csstools/value-no-unknown-custom-properties": [
      true,
      {
        importFrom: [
          "./node_modules/@elmethis/core/dist/tokens.css",
          "./src/index.css",
          "./src/App.css",
        ],
      },
    ],
    // kebab-case with optional BEM '__element' / '--modifier' suffix (e.g. today--holiday).
    "selector-class-pattern": [
      "^[a-z][a-z0-9]*(?:-[a-z0-9]+)*(?:__[a-z0-9]+(?:-[a-z0-9]+)*)?(?:--[a-z0-9]+(?:-[a-z0-9]+)*)?$",
      {
        message:
          "Expected class selector to be kebab-case with optional BEM '__element' and/or '--modifier' suffix",
      },
    ],
    "no-descending-specificity": null,
  },
  ignoreFiles: ["dist/**", "node_modules/**"],
};
