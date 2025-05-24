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

## Quick Start

### Prerequisites

- Rust (latest stable)
- Node.js (latest LTS)
- wasm-pack
- pnpm (recommended) or npm

### Install and Build

```bash
pnpm install
pnpm build
```

### Run the Web Example

```bash
cd examples/web
pnpm dev
```

### Updating the WASM Dependency

After making changes to the Rust WASM code, rebuild and update the dependency:

```bash
cd bindings/wasm
wasm-pack build --target web
cd ../../examples/web
pnpm install
```

#### Why do we need to run install after build?

see [WASM DEPENDENCY RESOLUTION](./ADR/0002-wasm-dependency-resolution.md).

---

## More Information

For architectural decisions, workflow rationale, and advanced usage, see the [ADR directory](./ADR/0001-project-design.md).

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

### Development Server

The web example runs on a development server at http://localhost:3000. To start it:

```bash
cd examples/web
pnpm dev
```

The server features:

- Hot Module Replacement (HMR)
- Fast refresh for quick development
- TypeScript support
- Vite's optimized build process

### Building the Core Library

```bash
cd core
cargo build
```

### Building WebAssembly Bindings

```bash
cd bindings/wasm
wasm-pack build --target web
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

MIT
