# ADR 0004: SVG as Source of Truth for QR Code Rendering

## Status
Accepted

## Context

The project needs to support multiple output formats for QR codes (SVG, PNG, JPEG) while maintaining consistent rendering across all formats. We need to decide on a primary rendering format that will serve as the source of truth, from which other formats can be derived.

## Decision

We will use SVG as the source of truth for QR code rendering, with the following approach:

1. **Primary SVG Renderer**: Implement a robust SVG renderer that handles all visual aspects of the QR code:
   - Module shapes (square, rounded, dot, triangle)
   - Finder pattern customization
   - Data module customization
   - Color schemes
   - Size and scaling

2. **Format Conversion**: Implement format conversion from SVG to other formats:
   - SVG → PNG: Using a headless browser or SVG rendering library
   - SVG → JPEG: Convert from PNG with appropriate compression
   - Future formats: Add new conversion paths as needed

3. **Rendering Pipeline**:
   ```
   QR Matrix → SVG Renderer → SVG → Format Converter → Target Format
   ```

## Consequences

### Positive
- **Consistency**: All output formats will be visually identical since they're derived from the same SVG
- **Flexibility**: SVG's vector nature allows for high-quality scaling to any size
- **Maintainability**: Only one rendering implementation needs to be maintained
- **Extensibility**: New visual features only need to be implemented in the SVG renderer
- **Quality**: Vector-based rendering ensures crisp output at any resolution

### Negative
- **Performance**: Additional conversion step required for non-SVG formats
- **Dependencies**: Need to maintain format conversion libraries
- **Complexity**: More complex build process for non-SVG formats

### Mitigations
- Cache converted formats to avoid repeated conversions
- Use efficient SVG parsing and rendering libraries
- Implement format conversion as optional features

## Alternatives Considered

1. **Direct Format Rendering**:
   - Implement separate renderers for each format
   - Rejected due to maintenance burden and potential inconsistencies

2. **Canvas-based Approach**:
   - Use HTML Canvas as the source of truth
   - Rejected due to platform dependencies and limited vector capabilities

3. **Raster-first Approach**:
   - Use PNG as the source of truth
   - Rejected due to loss of vector capabilities and scaling limitations

## Implementation Notes

1. **SVG Renderer**:
   - Keep the current SVG rendering implementation in `svg_rendering.rs`
   - Ensure it supports all required visual features
   - Add comprehensive tests for visual output

2. **Format Conversion**:
   - Use `resvg` or similar for SVG to PNG conversion
   - Use `image` crate for PNG to JPEG conversion
   - Implement conversion as separate modules

3. **API Design**:
   ```rust
   pub fn render_qr_code(
       data: &str,
       config: &QrRenderConfig,
       format: OutputFormat,
   ) -> Result<Vec<u8>, RenderError>
   ```

## References
- [SVG Specification](https://www.w3.org/TR/SVG2/)
- [resvg crate](https://crates.io/crates/resvg)
- [image crate](https://crates.io/crates/image) 