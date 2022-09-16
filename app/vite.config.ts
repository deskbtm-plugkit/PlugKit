import { defineConfig } from 'vite';
import { resolve } from 'path';
import react from '@vitejs/plugin-react';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  // To include built-in apps
  root: resolve(__dirname, '../'),
  publicDir: resolve(__dirname, 'public'),
  envDir: __dirname,
  build: {
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
      },
    },
  },
});
