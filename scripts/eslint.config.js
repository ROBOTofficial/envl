import prettier from "eslint-config-prettier";
import js from "@eslint/js";
import globals from "globals";
import tseslint from "typescript-eslint";
import { includeIgnoreFile } from "@eslint/compat";
import { defineConfig } from "eslint/config";
import { fileURLToPath } from "node:url";

const gitignorePath = fileURLToPath(new URL("./.gitignore", import.meta.url));

export default defineConfig([
	includeIgnoreFile(gitignorePath),
	prettier,
	{
		files: ["**/*.{js,mjs,cjs,ts}"],
		plugins: { js },
		extends: ["js/recommended"]
	},
	{
		files: ["**/*.{js,mjs,cjs,ts}"],
		languageOptions: { globals: globals.browser }
	},
	tseslint.configs.recommended
]);
