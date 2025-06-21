import { useEffect, useReducer, useState } from 'react';
import init, { render_qr_svg, QrConfig } from "@qrcode/wasm";

// Feature flags configuration
const FEATURE_FLAGS = {
  FINDER_SHAPES: {
    Square: true,
    Dot: false, // Disabled until implemented
    Rounded: false, // Disabled until implemented
    Triangle: false, // Disabled until implemented
  },
  DATA_SHAPES: {
    Square: true,
    Dot: true,
    Rounded: false, // Disabled until implemented
    Triangle: false, // Disabled until implemented
  },
  COLOR_CUSTOMIZATION: true,
} as const;

type QrRenderConfig = {
  finderShape: string;
  dataShape: string;
  finderColor: string;
  dataColor: string;
};

// Helper function to get available options based on feature flags
const getAvailableOptions = (type: 'FINDER_SHAPES' | 'DATA_SHAPES') => {
  const flagGroup = type === 'FINDER_SHAPES' ? FEATURE_FLAGS.FINDER_SHAPES : FEATURE_FLAGS.DATA_SHAPES;
  return Object.entries(flagGroup)
    .filter(([_, enabled]) => enabled)
    .map(([shape]) => shape);
};

const defaultConfig: QrRenderConfig = {
  finderShape: getAvailableOptions('FINDER_SHAPES')[0] || 'Square',
  dataShape: getAvailableOptions('DATA_SHAPES')[0] || 'Square',
  finderColor: '#000000',
  dataColor: '#000000'
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
  const [url, setUrl] = useState('https://google.com');
  const [qrCode, setQrCode] = useState<string>('');
  const [config, dispatch] = useReducer(qrReducer, defaultConfig);

  useEffect(() => {
    const initializeWasm = async () => {
      await init();
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
      console.log('finderColor', config.finderColor);
      console.log('dataColor', config.dataColor);
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
      <h1>QR Code Generator - WIP</h1>
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
              {getAvailableOptions('FINDER_SHAPES').map((shape) => (
                <option key={shape} value={shape}>{shape}</option>
              ))}
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
              {getAvailableOptions('DATA_SHAPES').map((shape) => (
                <option key={shape} value={shape}>{shape}</option>
              ))}
            </select>
          </label>
        </div>
        {FEATURE_FLAGS.COLOR_CUSTOMIZATION && (
          <>
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
          </>
        )}
      </div>
      <div id="qr-output" dangerouslySetInnerHTML={{ __html: qrCode }} />
    </div>
  );
} 