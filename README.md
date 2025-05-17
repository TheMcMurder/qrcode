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

### WebAssembly Package Dependency

> **Note:** The web example depends on the WASM package using a `file:` dependency pointing to the build output (`bindings/wasm/pkg`). This is necessary because the workspace protocol links the source directory, not the build output, and the generated JS/WASM package (with its own `package.json`) lives in `pkg/` after running `wasm-pack build`.

#### Why not `workspace:*`?
- The workspace protocol (e.g., `"workspace:*"`) links the root of the package, not the `pkg/` directory where the WASM build output lives.
- The web example needs the generated JS/WASM files and the `package.json` from `pkg/`, so we use a file dependency:  
  `"@qrcode/wasm": "file:../../bindings/wasm/pkg"`

#### Workflow Steps
1. **Build the WASM package:**
   ```bash
   cd bindings/wasm
   wasm-pack build --target web
   ```
2. **Update dependencies in the web example:**
   ```bash
   cd ../../examples/web
   pnpm install
   ```
   This ensures the web example picks up the latest WASM build output.

3. **Run the development server:**
   ```bash
   pnpm dev
   ```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

MIT 