import eslintPluginAstro from "eslint-plugin-astro";
import tsEslint from "@typescript-eslint/eslint-plugin";
import tsEslintParser from "@typescript-eslint/parser";

export default [
  ...eslintPluginAstro.configs.recommended,
  {
    files: ["**/*.ts", "**/*.tsx"],
    ignores: ["**/node_modules/**", "**/dist/**"],
    languageOptions: { parser: tsEslintParser },
  },
  {
    plugins: {
      "@typescript-eslint": tsEslint,
    },
    rules: {
      "no-unused-vars": "off",
      "@typescript-eslint/no-unused-vars": [
        "warn",
        {
          varsIgnorePattern: "^_",
          argsIgnorePattern: "^_",
          destructuredArrayIgnorePattern: "^_",
        },
      ],
    },
  },
];
