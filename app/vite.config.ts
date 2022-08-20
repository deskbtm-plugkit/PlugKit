import { defineConfig } from 'vite';
import { resolve } from 'path';
import react from '@vitejs/plugin-react';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  root: resolve(__dirname, 'src/home'),
  publicDir: resolve(__dirname, 'public'),
  build: {
    rollupOptions: {
      input: {
        index: resolve(__dirname, 'src/home/index.html'),
        setting: resolve(__dirname, 'src/setting/index.html'),
      },
    },
  },
});
