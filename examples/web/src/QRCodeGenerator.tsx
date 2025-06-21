import { useEffect, useReducer, useState } from "react";
import { Disclaimer } from "./Disclaimer";
import init from "@qrcode/wasm";
import { QrRenderConfig } from "./types";
import { SVGQRCode } from "./SVGQRCode";
import { PNGQRCode } from "./PNGQRCode";
import { JPGQRCode } from "./JPGQRCode";

// Feature flags configuration
const FEATURE_FLAGS = {
  FINDER_SHAPES: {
    Square: true,
    Dot: true,
    Rounded: true,
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

// Helper function to get available options based on feature flags
const getAvailableOptions = (type: "FINDER_SHAPES" | "DATA_SHAPES") => {
  const flagGroup =
    type === "FINDER_SHAPES"
      ? FEATURE_FLAGS.FINDER_SHAPES
      : FEATURE_FLAGS.DATA_SHAPES;
  return Object.entries(flagGroup)
    .filter(([_, enabled]) => enabled)
    .map(([shape]) => shape);
};

const defaultConfig: QrRenderConfig = {
  finderShape: getAvailableOptions("FINDER_SHAPES")[0] || "Square",
  dataShape: getAvailableOptions("DATA_SHAPES")[0] || "Square",
  finderColor: "#000000",
  dataColor: "#000000",
};

type QrAction =
  | { type: "SET_FINDER_SHAPE"; payload: string }
  | { type: "SET_DATA_SHAPE"; payload: string }
  | { type: "SET_FINDER_COLOR"; payload: string }
  | { type: "SET_DATA_COLOR"; payload: string };

function qrReducer(state: QrRenderConfig, action: QrAction): QrRenderConfig {
  switch (action.type) {
    case "SET_FINDER_SHAPE":
      return { ...state, finderShape: action.payload };
    case "SET_DATA_SHAPE":
      return { ...state, dataShape: action.payload };
    case "SET_FINDER_COLOR":
      return { ...state, finderColor: action.payload };
    case "SET_DATA_COLOR":
      return { ...state, dataColor: action.payload };
    default:
      return state;
  }
}

export function QRCodeGenerator() {
  const [url, setUrl] = useState("https://google.com");
  const [isInitialized, setIsInitialized] = useState(false);
  const [config, dispatch] = useReducer(qrReducer, defaultConfig);

  useEffect(() => {
    const initializeWasm = async () => {
      await init();
      setIsInitialized(true);
    };
    initializeWasm();
  }, []);

  return (
    <div>
      <h1>QR Code Generator - WIP</h1>
      <Disclaimer />
      <div>
        <input
          id="qr-input"
          type="text"
          placeholder="Enter a URL"
          value={url}
          onChange={(e) => setUrl(e.target.value)}
        />
      </div>
      <div>
        <h3>Configuration</h3>
        <div>
          <label>
            Finder Shape:
            <select
              value={config.finderShape}
              onChange={(e) =>
                dispatch({ type: "SET_FINDER_SHAPE", payload: e.target.value })
              }
            >
              {getAvailableOptions("FINDER_SHAPES").map((shape) => (
                <option key={shape} value={shape}>
                  {shape}
                </option>
              ))}
            </select>
          </label>
        </div>
        <div>
          <label>
            Data Shape:
            <select
              value={config.dataShape}
              onChange={(e) =>
                dispatch({ type: "SET_DATA_SHAPE", payload: e.target.value })
              }
            >
              {getAvailableOptions("DATA_SHAPES").map((shape) => (
                <option key={shape} value={shape}>
                  {shape}
                </option>
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
                  onChange={(e) =>
                    dispatch({
                      type: "SET_FINDER_COLOR",
                      payload: e.target.value,
                    })
                  }
                />
              </label>
            </div>
            <div>
              <label>
                Data Color:
                <input
                  type="color"
                  value={config.dataColor}
                  onChange={(e) =>
                    dispatch({
                      type: "SET_DATA_COLOR",
                      payload: e.target.value,
                    })
                  }
                />
              </label>
            </div>
          </>
        )}
      </div>
      {isInitialized && (
        <div
          style={{
            display: "grid",
            gridTemplateColumns: "repeat(3, 1fr)",
            gap: "1rem",
            textAlign: "center",
          }}
        >
          <div>
            <h3>SVG</h3>
            <SVGQRCode url={url} config={config} />
          </div>
          <div>
            <h3>PNG</h3>
            <PNGQRCode url={url} config={config} />
          </div>
          <div>
            <h3>JPG</h3>
            <JPGQRCode url={url} config={config} />
          </div>
        </div>
      )}
    </div>
  );
}
