import { useEffect, useState } from 'react';
import init, { hello, render_qr_svg } from "@qrcode/wasm";

export function QRCodeGenerator() {
  const [message, setMessage] = useState<string>('');
  const [url, setUrl] = useState('https://google.com');
  const [qrCode, setQrCode] = useState<string>('');

  useEffect(() => {
    const initializeWasm = async () => {
      await init();
      setMessage(hello());
    };
    initializeWasm();
  }, []);

  const handleGenerateQR = () => {
    const trimmedUrl = url.trim();
    if (!trimmedUrl) {
      setQrCode('<em>Please enter a URL.</em>');
      return;
    }
    try {
      console.log('trimmedUrl', trimmedUrl)
      const svg = render_qr_svg(trimmedUrl);
      setQrCode(svg);
    } catch (e) {
      setQrCode('<em>Failed to generate QR code.</em>');
      console.error(e);
    }
  };

  return (
    <div>
      <h1>QR Code Generator</h1>
      <p>{message}</p>
      <input
        id="qr-input"
        type="text"
        placeholder="Enter a URL This is react"
        value={url}
        onChange={(e) => setUrl(e.target.value)}
      />
      <button id="qr-generate" onClick={handleGenerateQR}>
        Generate QR Code this is react
      </button>
      <div id="qr-output" dangerouslySetInnerHTML={{ __html: qrCode }} />
    </div>
  );
} 