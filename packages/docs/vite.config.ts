import { defineConfig } from 'vite'
import react, { reactCompilerPreset } from '@vitejs/plugin-react'
import babel from '@rolldown/plugin-babel'

// Served from GitHub Pages under the project subpath.
// https://vite.dev/config/
export default defineConfig({
  base: '/jp-holidays/',
  plugins: [
    react(),
    babel({ presets: [reactCompilerPreset()] })
  ],
})
