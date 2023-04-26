import { defineConfig } from 'vite';
import { resolve } from 'path';
import react from '@vitejs/plugin-react';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    port: 1420,
    strictPort: true,
  },
  // To include built-in apps
  root: resolve(__dirname, '../'),
  publicDir: resolve(__dirname, 'public'),
  envDir: __dirname,
  clearScreen: false,
  build: {
    target: process.env.TAURI_PLATFORM == 'windows' ? 'chrome105' : 'safari13',
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG,
    outDir: resolve(__dirname, 'dist'),
    emptyOutDir: true,
    rollupOptions: {
      input: {
        index: resolve(__dirname, 'src/index.html'),
        setting: resolve(__dirname, 'src/setting/index.html'),
        'builtin-wallpaper': resolve(
          __dirname,
          '../built-in/wallpaper/src/index.html',
        ),
        'builtin-live2d': resolve(
          __dirname,
          '../built-in/live2d/src/index.html',
        ),
      },
    },
  },
});
