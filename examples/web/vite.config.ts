import { defineConfig } from "vite";
import { resolve } from "path";
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  base: process.env.NODE_ENV === 'production' ? '/qrcode/' : '/',
  server: {
    port: 3000,
  },
  build: {
    outDir: 'dist',
    assetsDir: 'assets',
    rollupOptions: {
      output: {
        manualChunks: {
          vendor: ['react', 'react-dom'],
        },
      },
    },
  },
  resolve: {
    alias: {
      "@qrcode/wasm": resolve(__dirname, "../../bindings/wasm/pkg"),
    },
  },
  optimizeDeps: {
    exclude: ["@qrcode/wasm"],
  },
});
