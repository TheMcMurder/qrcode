import { QrConfig, render_qr_jpeg } from "@qrcode/wasm";
import { useEffect, useState } from "react";
import { QrRenderConfig } from "./types.js";

export function JPGQRCode({
  url,
  config,
}: {
  url: string;
  config: QrRenderConfig;
}) {
  const [qrCode, setQrCode] = useState<string>("");
  const [error, setError] = useState<string>("");

  useEffect(() => {
    handleGenerateQR();
  }, [url, config]);

  const handleGenerateQR = () => {
    setError("");
    setQrCode("");

    const trimmedUrl = url.trim();
    if (!trimmedUrl) {
      return;
    }

    try {
      const qrConfig = new QrConfig(
        config.finderShape,
        config.dataShape,
        config.finderColor,
        config.dataColor,
      );
      const png = render_qr_jpeg(trimmedUrl, qrConfig);
      setQrCode(png);
    } catch (e) {
      setError("Failed to generate QR code.");
      console.error(e);
    }
  };

  if (error) {
    return (
      <>
        <div id="qr-output">
          <em>{error}</em>
        </div>
      </>
    );
  }

  if (!url.trim()) {
    return (
      <>
        <div id="qr-output">
          <em>Please enter a URL.</em>
        </div>
      </>
    );
  }

  if (!qrCode) {
    return (
      <>
        <div id="qr-output">
          <em>Generating...</em>
        </div>
      </>
    );
  }

  return (
    <>
      <div id="qr-output">
        <img src={`data:image/jpeg;base64,${qrCode}`} alt="QR Code JPG" />
      </div>
    </>
  );
}
