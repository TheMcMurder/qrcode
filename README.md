# QR Code Generator

A high-performance, cross-platform QR code generator written in Rust with WebAssembly support. This project aims to provide a robust, efficient, and flexible solution for generating QR codes across different platforms and use cases.

## Project Goals

- Create a performant QR code generation library in Rust
- Provide WebAssembly bindings for web applications
- Support multiple output formats (PNG, SVG, etc.)
- Enable customization of QR code appearance and properties
- Maintain high code quality and comprehensive test coverage
- Offer a simple, intuitive API for developers

## Features

- 🚀 High-performance QR code generation using Rust
- 🌐 WebAssembly support for web applications
- 🎨 Customizable QR code appearance
- 📦 Multiple output format support
- 🔒 Error correction levels
- 🧪 Comprehensive test suite
- 📚 Well-documented API

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