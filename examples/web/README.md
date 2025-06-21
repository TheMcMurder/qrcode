# QR Code Generator Web Example

This is a web-based example of the QR Code Generator library, demonstrating the WebAssembly integration and customizable QR code rendering.

## Features

- 🚀 High-performance QR code generation using Rust + WebAssembly
- 🎨 Customizable appearance (shapes, colors)
- 📱 Responsive design
- 🌐 Live demo available on GitHub Pages

## Live Demo

Visit the live demo at: [https://themcmurder.github.io/qrcode/](https://themcmurder.github.io/qrcode/)

## Local Development

### Prerequisites

- Node.js (latest LTS)
- pnpm (recommended) or npm
- Rust (latest stable)
- wasm-pack

### Setup

1. Install dependencies:

   ```bash
   pnpm install
   ```

2. Build WASM bindings:

   ```bash
   cd ../../bindings/wasm
   wasm-pack build --target web
   cd ../../examples/web
   pnpm install
   ```

3. Start development server:
   ```bash
   pnpm dev
   ```

The application will be available at `http://localhost:3000`.

### Building for Production

```bash
pnpm run build:gh-pages
```

This will create a production build optimized for GitHub Pages deployment.

## Deployment

This example is automatically deployed to GitHub Pages via GitHub Actions. The deployment process:

1. Builds the WASM bindings
2. Builds the web application
3. Deploys to GitHub Pages

### Manual Deployment

If you need to deploy manually:

1. Build the project:

   ```bash
   pnpm run build:gh-pages
   ```

2. The built files will be in the `dist/` directory

3. Upload the contents of `dist/` to your GitHub Pages branch

## Architecture

- **Frontend**: React + TypeScript + Vite
- **QR Code Generation**: Rust + WebAssembly
- **Styling**: CSS with modern features
- **Deployment**: GitHub Pages with GitHub Actions

## Customization

The example demonstrates various customization options:

- **Finder Shape**: Square, Dot, Rounded, Triangle
- **Data Shape**: Square, Dot, Rounded, Triangle
- **Colors**: Customizable finder and data colors
- **Input**: URL or text input for QR code generation

## Troubleshooting

### WASM Loading Issues

If you encounter WASM loading issues:

1. Ensure the WASM bindings are built:

   ```bash
   cd ../../bindings/wasm
   wasm-pack build --target web
   ```

2. Update the dependency:
   ```bash
   cd ../../examples/web
   pnpm install
   ```

### Build Issues

If the build fails:

1. Clear node_modules and reinstall:

   ```bash
   pnpm clean
   pnpm install
   ```

2. Ensure all dependencies are up to date:
   ```bash
   pnpm update
   ```

## Contributing

This example is part of the larger QR Code Generator project. See the main [README](../../README.md) for contribution guidelines.
