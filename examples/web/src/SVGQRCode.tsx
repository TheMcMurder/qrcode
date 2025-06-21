import { render_qr_svg, QrConfig } from "@qrcode/wasm";
import { useEffect, useState } from "react";
import { QrRenderConfig } from "./types.js";

export function SVGQRCode({
  url,
  config,
}: {
  url: string;
  config: QrRenderConfig;
}) {
  const [qrCode, setQrCode] = useState<string>("");

  useEffect(() => {
    handleGenerateQR();
  }, [url, config]);

  const handleGenerateQR = () => {
    const trimmedUrl = url.trim();
    if (!trimmedUrl) {
      setQrCode("<em>Please enter a URL.</em>");
      return;
    }
    try {
      console.log("finderColor", config.finderColor);
      console.log("dataColor", config.dataColor);
      const qrConfig = new QrConfig(
        config.finderShape,
        config.dataShape,
        config.finderColor,
        config.dataColor,
      );
      const svg = render_qr_svg(trimmedUrl, qrConfig);
      setQrCode(svg);
    } catch (e) {
      setQrCode("<em>Failed to generate QR code.</em>");
      console.error(e);
    }
  };
  return (
    <>
      <div>SVG rendering</div>
      <div id="qr-output">
        <div dangerouslySetInnerHTML={{ __html: qrCode }} />
      </div>
    </>
  )
}
