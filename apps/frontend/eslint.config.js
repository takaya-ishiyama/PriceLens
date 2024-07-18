import globals from "globals";
import pluginJs from "@eslint/js";
import tseslint from "typescript-eslint";
import pluginReactConfig from "eslint-plugin-react/configs/recommended.js";
import { fixupConfigRules } from "@eslint/compat";
import pluginReact from 'eslint-plugin-react';
import reactHooks from 'eslint-plugin-react-hooks';


export default [
  {
    files: ["**/*.{ts,tsx}"],
    plugins:{
      react: pluginReact,
      'react-hooks':reactHooks
    },
    rules: {      
      ...reactHooks.configs.recommended.rules,
      // Ensure you explicitly include the exhaustive-deps rule
      'react-hooks/exhaustive-deps': 'warn'
    },
  },
  { languageOptions: { parserOptions: { ecmaFeatures: { jsx: true } } } },
  {languageOptions: { globals: globals.browser }},
  {},
  pluginJs.configs.recommended,
  ...tseslint.configs.recommended,
  ...fixupConfigRules(pluginReactConfig),
];
