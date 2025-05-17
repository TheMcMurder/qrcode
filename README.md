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

## Build System

This project uses [Turborepo](https://turbo.build/repo) to manage the build pipeline and dependencies between packages. Turborepo helps us:

- Manage dependencies between Rust and JavaScript/TypeScript packages
- Cache build outputs for faster subsequent builds
- Run builds in parallel where possible
- Maintain consistent build environments

### Key Concepts

- **Workspaces**: The project is organized into workspaces (core, bindings, examples)
- **Pipeline**: Defined in `turbo.json`, orchestrates the build process
- **Caching**: Build outputs are cached to speed up development
- **Dependencies**: Automatic handling of workspace dependencies

## Development Setup

### Prerequisites

- Rust (latest stable)
- Node.js (latest LTS)
- wasm-pack (for WebAssembly builds)
- pnpm (recommended) or npm

### Initial Setup

```bash
# Install dependencies
pnpm install

# Build all packages
pnpm build

# Run development environment
pnpm dev
```

### Individual Package Development

You can work on specific packages using Turborepo's filtering:

```bash
# Build only the core library
pnpm build --filter=core

# Build and watch the web example
pnpm dev --filter=web-example
```

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

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

MIT 