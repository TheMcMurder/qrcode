# ADR 0003: Use of Existing QR Code Library with Custom Renderer

## Status
Accepted

## Context

The project requires generating QR codes with advanced and customizable rendering options (such as gradients, rounded corners, or other visual effects) that go beyond the capabilities of most existing Rust QR code libraries. However, implementing the QR code encoding algorithm from scratch would be time-consuming and error-prone.

## Decision

We decided to use the well-maintained [`qrcode`](https://crates.io/crates/qrcode) crate to generate the QR code matrix (the pattern of dark and light modules). This allows us to:

- Leverage a robust, tested implementation for QR code data encoding and error correction.
- Avoid re-implementing the complex QR code specification.
- Focus our efforts on custom rendering and output features.

For rendering, we build our own parser and renderer on top of the matrix provided by the `qrcode` crate. This gives us full control over the output format (SVG, PNG, etc.) and enables advanced visual features not supported by the library's built-in renderers.

## Consequences

- We benefit from the reliability and performance of the `qrcode` crate for matrix generation.
- We are free to implement any rendering logic or visual style, unconstrained by the library's built-in output options.
- Maintenance is simplified: we only need to update our renderer if we want new features, while the underlying QR code logic is maintained by the open source community.
- If the `qrcode` crate changes its matrix API, we may need to update our integration.

## Alternatives Considered

- **Implementing the QR code algorithm from scratch:** Rejected due to complexity and risk of subtle bugs.
- **Using the library's built-in renderers:** Rejected because they do not support advanced visual features (e.g., gradients, rounded corners).

## References
- [qrcode crate documentation](https://docs.rs/qrcode/)
- See also: [ADR 0001 - Project Design and Architecture](./0001-project-design.md) 