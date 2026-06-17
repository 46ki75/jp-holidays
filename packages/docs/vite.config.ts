/// <reference types="vitest/config" />
import { defineConfig } from 'vite'
import react, { reactCompilerPreset } from '@vitejs/plugin-react'
import babel from '@rolldown/plugin-babel'

// The React Compiler runs via Babel at build time. It is skipped under Vitest
// so specs exercise the uncompiled source (the compiled output is covered by
// the production build).
const reactCompiler = process.env.VITEST
  ? []
  : [babel({ presets: [reactCompilerPreset()] })]

// Served from GitHub Pages under the project subpath.
// https://vite.dev/config/
export default defineConfig({
  base: '/jp-holidays/',
  plugins: [react(), ...reactCompiler],
  // Unit layer: pure logic plus component specs rendered in a DOM emulator
  // (happy-dom) via React Testing Library. No real browser is involved.
  test: {
    globals: true,
    environment: 'happy-dom',
    setupFiles: ['./vitest.setup.ts'],
    // Inline the elmethis packages so Vite transforms them — they ship CSS
    // imports that Node's loader can't handle when left externalized.
    server: { deps: { inline: [/@elmethis\//] } },
    coverage: {
      provider: 'v8',
      include: ['src/**/*.{ts,tsx}'],
      exclude: ['src/**/*.test.{ts,tsx}', 'src/main.tsx', 'src/vite-env.d.ts'],
      reporter: ['text', 'html'],
      reportsDirectory: 'coverage',
    },
  },
})
