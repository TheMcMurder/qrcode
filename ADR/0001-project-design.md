# ADR 0001: Project Design and Architecture

## Status

Accepted

## Context

This project aims to provide a high-performance, cross-platform QR code generator with a focus on flexibility, efficiency, and ease of integration across different environments. The design leverages Rust for its performance and safety, WebAssembly (WASM) for web compatibility, and a monorepo structure managed by Turborepo for streamlined development and builds.

## Decision

- **Core Logic in Rust:**

  - The core QR code generation logic is implemented in Rust for maximum performance, safety, and portability.
  - Rust's ecosystem enables robust testing, error handling, and future extensibility (e.g., supporting more output formats).

- **WebAssembly Bindings:**

  - WebAssembly bindings are provided via a dedicated package (`bindings/wasm`) using `wasm-pack` and `wasm-bindgen`.
  - This enables the core logic to be reused in web applications with minimal overhead and maximum performance.

- **Monorepo with Turborepo:**

  - The project is organized as a monorepo using Turborepo, with separate packages for the core library, WASM bindings, and web examples.
  - Turborepo manages build pipelines, caching, and workspace dependencies, improving developer experience and CI efficiency.

- **Web Example:**

  - A web example (`examples/web`) demonstrates how to consume the WASM package in a modern frontend stack (Vite + TypeScript).
  - This serves as both a reference implementation and a testbed for integration.

- **Future Bindings:**
  - We plan to support Android and iOS with future bindings as well.

## Consequences

- The project is easy to extend with new bindings (e.g., Node.js, Python) by adding new packages to the monorepo.
- The separation of concerns (core logic, bindings, examples) keeps the codebase maintainable and clear.
- Contributors can work on individual packages or the whole project, leveraging Turborepo's filtering and caching.
- The use of Rust and WASM ensures high performance and cross-platform compatibility.

## Alternatives Considered

- Implementing the core logic directly in JavaScript/TypeScript: rejected due to performance and maintainability concerns.
- Using separate repositories for each package: rejected in favor of a monorepo for easier dependency management and unified workflows.

## References

- See the [README](../README.md) for quick start and usage instructions.
- See other ADRs for specific architectural decisions (e.g., WASM dependency resolution).
