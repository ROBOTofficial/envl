import { defineConfig } from "tsup";

export default defineConfig({
    entry: ["src/index.ts"],
    dts: true,
    format: ["esm", "cjs", "iife"],
    tsconfig: "tsconfig.json",
    clean: true
});
