use std::io::Cursor;
use thiserror::Error;
use image::{ImageBuffer, Rgba};
use resvg;
use usvg::{Tree, Options, TreeParsing};

/// Custom error type for format conversion operations
#[derive(Error, Debug)]
pub enum FormatConversionError {
    #[error("Failed to parse SVG: {0}")]
    SvgParseError(String),
    
    #[error("Failed to render SVG: {0}")]
    SvgRenderError(String),
    
    #[error("Failed to encode image: {0}")]
    ImageEncodeError(String),
}

/// Supported raster output formats
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RasterFormat {
    Png,
    Jpeg,
}

/// The actual output data from QR code rendering
#[derive(Debug)]
pub enum QrCodeOutput {
    /// SVG string representation
    Svg(String),
    /// Raster image data with format information
    Raster(Vec<u8>, RasterFormat),
}

/// Complete QR code rendering result
#[derive(Debug)]
pub struct QrCodeResult {
    /// The rendered QR code data
    pub data: QrCodeOutput,
    /// Width of the rendered QR code in pixels
    pub width: u32,
    /// Height of the rendered QR code in pixels
    pub height: u32,
}

/// Converts an SVG string to a PNG image buffer
/// 
/// # Arguments
/// * `svg_string` - The SVG content as a string
/// * `size` - The desired output size in pixels
/// 
/// # Returns
/// * `Result<ImageBuffer<Rgba<u8>, Vec<u8>>>` - The rendered PNG image buffer
fn svg_to_png(svg_string: &str, size: u32) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, FormatConversionError> {
    // Parse the SVG string into a usvg Tree
    let tree = Tree::from_str(svg_string, &Options::default())
        .map_err(|e| FormatConversionError::SvgParseError(e.to_string()))?;
    
    // Create a new image buffer with the specified size
    let mut pixmap = resvg::tiny_skia::Pixmap::new(size, size)
        .ok_or_else(|| FormatConversionError::SvgRenderError("Failed to create pixmap".to_string()))?;
    
    // Convert to resvg Tree and render
    let rtree = resvg::Tree::from_usvg(&tree);
    rtree.render(
        resvg::tiny_skia::Transform::identity(),
        &mut pixmap.as_mut()
    );
    
    // Convert the pixmap to an image buffer
    let image_buffer = ImageBuffer::from_raw(size, size, pixmap.data().to_vec())
        .ok_or_else(|| FormatConversionError::SvgRenderError("Failed to create image buffer".to_string()))?;
    
    Ok(image_buffer)
}

/// Converts an SVG string to the specified output format
/// 
/// # Arguments
/// * `svg_string` - The SVG content as a string
/// * `format` - The desired output format
/// * `size` - The desired output size in pixels
/// 
/// # Returns
/// * `Result<QrCodeResult>` - The rendered QR code result
pub fn convert_svg_to_format(
    svg_string: &str,
    format: RasterFormat,
    size: u32,
) -> Result<QrCodeResult, FormatConversionError> {
    // First convert SVG to PNG buffer
    let image_buffer = svg_to_png(svg_string, size)?;
    
    // Create a buffer to hold the encoded image data
    let mut output_buffer = Vec::new();
    let mut cursor = Cursor::new(&mut output_buffer);
    
    // Encode the image buffer to the requested format
    match format {
        RasterFormat::Png => {
            image_buffer
                .write_to(&mut cursor, image::ImageFormat::Png)
                .map_err(|e| FormatConversionError::ImageEncodeError(e.to_string()))?;
        }
        RasterFormat::Jpeg => {
            image_buffer
                .write_to(&mut cursor, image::ImageFormat::Jpeg)
                .map_err(|e| FormatConversionError::ImageEncodeError(e.to_string()))?;
        }
    }
    
    Ok(QrCodeResult {
        data: QrCodeOutput::Raster(output_buffer, format),
        width: size,
        height: size,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_svg_to_png_conversion() {
        // Create a simple SVG string
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
            <rect width="100" height="100" fill="white"/>
            <rect x="10" y="10" width="80" height="80" fill="black"/>
        </svg>"#;
        
        // Convert to PNG
        let result = convert_svg_to_format(svg, RasterFormat::Png, 100);
        assert!(result.is_ok());
        
        let qr_result = result.unwrap();
        match qr_result.data {
            QrCodeOutput::Raster(data, format) => {
                assert_eq!(format, RasterFormat::Png);
                assert!(data.starts_with(b"\x89PNG\r\n\x1a\n"));
            }
            _ => panic!("Expected Raster output"),
        }
        assert_eq!(qr_result.width, 100);
        assert_eq!(qr_result.height, 100);
    }
} 