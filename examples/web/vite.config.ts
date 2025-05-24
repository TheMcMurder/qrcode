import { defineConfig } from "vite";
import { resolve } from "path";

export default defineConfig({
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
