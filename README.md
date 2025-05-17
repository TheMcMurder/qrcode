# QR Code Generator

A cross-platform QR code generator written in Rust with WebAssembly support.

## Project Structure

```
qrcode/
├── core/                 # Core Rust library
├── bindings/            # Platform-specific bindings
│   └── wasm/           # WebAssembly bindings
├── examples/           # Example implementations
│   └── web/           # Web example using Vite
└── docs/              # Documentation
```

## Development Setup

### Prerequisites

- Rust (latest stable)
- Node.js (latest LTS)
- wasm-pack (for WebAssembly builds)

### Building the Core Library

```bash
cd core
cargo build
```

### Building WebAssembly Bindings

```bash
cd bindings/wasm
wasm-pack build
```

### Running the Web Example

```bash
cd examples/web
npm install
npm run dev
```

## License

MIT 