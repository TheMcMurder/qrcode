import { useEffect, useReducer, useState } from 'react';
import init, { hello, render_qr_svg, QrConfig } from "@qrcode/wasm";

type QrRenderConfig = {
  finderShape: string;
  dataShape: string;
  finderColor: string;
  dataColor: string;
};

const defaultConfig: QrRenderConfig = {
  finderShape: 'Square',
  dataShape: 'Dot',
  finderColor: 'green',
  dataColor: 'red'
};

type QrAction = 
  | { type: 'SET_FINDER_SHAPE'; payload: string }
  | { type: 'SET_DATA_SHAPE'; payload: string }
  | { type: 'SET_FINDER_COLOR'; payload: string }
  | { type: 'SET_DATA_COLOR'; payload: string };

function qrReducer(state: QrRenderConfig, action: QrAction): QrRenderConfig {
  switch (action.type) {
    case 'SET_FINDER_SHAPE':
      return { ...state, finderShape: action.payload };
    case 'SET_DATA_SHAPE':
      return { ...state, dataShape: action.payload };
    case 'SET_FINDER_COLOR':
      return { ...state, finderColor: action.payload };
    case 'SET_DATA_COLOR':
      return { ...state, dataColor: action.payload };
    default:
      return state;
  }
}

export function QRCodeGenerator() {
  const [message, setMessage] = useState<string>('');
  const [url, setUrl] = useState('https://google.com');
  const [qrCode, setQrCode] = useState<string>('');
  const [config, dispatch] = useReducer(qrReducer, defaultConfig);

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
      const qrConfig = new QrConfig(
        config.finderShape,
        config.dataShape,
        config.finderColor,
        config.dataColor
      );
      const svg = render_qr_svg(trimmedUrl, qrConfig);
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
      <div>
        <input
          id="qr-input"
          type="text"
          placeholder="Enter a URL"
          value={url}
          onChange={(e) => setUrl(e.target.value)}
        />
        <button id="qr-generate" onClick={handleGenerateQR}>
          Generate QR Code
        </button>
      </div>
      <div>
        <h3>Configuration</h3>
        <div>
          <label>
            Finder Shape:
            <select
              value={config.finderShape}
              onChange={(e) => dispatch({ type: 'SET_FINDER_SHAPE', payload: e.target.value })}
            >
              <option value="Square">Square</option>
              <option value="Dot">Dot</option>
              <option value="Rounded">Rounded</option>
              <option value="Triangle">Triangle</option>
            </select>
          </label>
        </div>
        <div>
          <label>
            Data Shape:
            <select
              value={config.dataShape}
              onChange={(e) => dispatch({ type: 'SET_DATA_SHAPE', payload: e.target.value })}
            >
              <option value="Square">Square</option>
              <option value="Dot">Dot</option>
              <option value="Rounded">Rounded</option>
              <option value="Triangle">Triangle</option>
            </select>
          </label>
        </div>
        <div>
          <label>
            Finder Color:
            <input
              type="color"
              value={config.finderColor}
              onChange={(e) => dispatch({ type: 'SET_FINDER_COLOR', payload: e.target.value })}
            />
          </label>
        </div>
        <div>
          <label>
            Data Color:
            <input
              type="color"
              value={config.dataColor}
              onChange={(e) => dispatch({ type: 'SET_DATA_COLOR', payload: e.target.value })}
            />
          </label>
        </div>
      </div>
      <div id="qr-output" dangerouslySetInnerHTML={{ __html: qrCode }} />
    </div>
  );
} 