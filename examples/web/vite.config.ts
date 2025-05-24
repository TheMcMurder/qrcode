import { defineConfig } from "vite";
import { resolve } from "path";
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  server: {
    port: 3000,
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
