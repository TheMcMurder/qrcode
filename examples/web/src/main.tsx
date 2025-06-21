import React from "react";
import { createRoot } from "react-dom/client";
import { QRCodeGenerator } from "./QRCodeGenerator";

const container = document.getElementById("app");
if (!container) {
  throw new Error("Root element not found");
}

const root = createRoot(container);
root.render(
  <React.StrictMode>
    <QRCodeGenerator />
  </React.StrictMode>,
);
