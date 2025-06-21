pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn hello() -> String {
    "Hello from Rust!".to_string()
}

mod svg_rendering;
mod format_conversion;

pub use svg_rendering::{render_qr_matrix_as_svg, DataShape, FinderShape, QrRenderConfig, FinderStyle, DataStyle};
pub use format_conversion::{RasterFormat, FormatConversionError, QrCodeOutput, QrCodeResult};

/// Renders a QR code in the specified format
/// 
/// # Arguments
/// * `data` - The data to encode in the QR code
/// * `config` - Optional rendering configuration
/// * `format` - The desired output format
/// * `size` - The desired output size in pixels
/// 
/// # Returns
/// * `Result<QrCodeResult>` - The rendered QR code result
pub fn render_qr_code(
    data: &str,
    config: Option<&QrRenderConfig>,
    format: RasterFormat,
    size: u32,
) -> Result<QrCodeResult, FormatConversionError> {
    // Generate the QR code matrix
    let matrix = generate_qr_matrix(data);
    
    // Render the matrix as SVG
    let svg = render_qr_matrix_as_svg(&matrix, config);
    
    // Convert the SVG to the requested format
    format_conversion::convert_svg_to_format(&svg, format, size)
}

/// Renders a QR code as SVG
/// 
/// # Arguments
/// * `data` - The data to encode in the QR code
/// * `config` - Optional rendering configuration
/// 
/// # Returns
/// * `QrCodeResult` - The rendered QR code result
pub fn render_qr_code_svg(
    data: &str,
    config: Option<&QrRenderConfig>,
) -> QrCodeResult {
    // Generate the QR code matrix
    let matrix = generate_qr_matrix(data);
    
    // Render the matrix as SVG
    let svg = render_qr_matrix_as_svg(&matrix, config);
    
    // Calculate size based on the matrix dimensions
    let size = (matrix.len() * 10) as u32; // 10 pixels per module
    
    QrCodeResult {
        data: QrCodeOutput::Svg(svg),
        width: size,
        height: size,
    }
}

// Generates a QR code matrix for input data (https://google.com)
// Returns a 2D vector of booleans, where true = dark module, false = light module
pub fn generate_qr_matrix(data_to_encode: &str) -> Vec<Vec<bool>> {
    // Import the qrcode crate
    use qrcode::QrCode;

    // The data to encode
    let url = data_to_encode;

    // Create the QR code (default error correction)
    let code = QrCode::new(url).expect("Failed to generate QR code");

    // Convert the QR code into a boolean matrix
    // Each module (pixel) is true (dark) or false (light)
    let width = code.width();
    let mut matrix = Vec::with_capacity(width);
    for y in 0..width {
        let mut row = Vec::with_capacity(width);
        for x in 0..width {
            // Convert Color to bool: true = dark, false = light
            let color = code[(x, y)];
            row.push(matches!(color, qrcode::Color::Dark));
        }
        matrix.push(row);
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        assert_eq!(hello(), "Hello from Rust!");
    }

    #[test]
    fn generates_qr_matrix() {
        let matrix = generate_qr_matrix("https://jedi.org");
        // The matrix should be square and non-empty
        assert!(!matrix.is_empty());
        let width = matrix.len();
        for row in &matrix {
            assert_eq!(row.len(), width);
        }
        // The matrix should contain both true and false values
        let flat: Vec<bool> = matrix.iter().flatten().cloned().collect();
        assert!(flat.contains(&true));
        assert!(flat.contains(&false));
    }

    #[test]
    fn renders_qr_code_in_different_formats() {
        // Test PNG rendering
        let png_result = render_qr_code(
            "https://jedi.org",
            None,
            RasterFormat::Png,
            200,
        );
        assert!(png_result.is_ok());
        let qr_result = png_result.unwrap();
        match qr_result.data {
            QrCodeOutput::Raster(data, format) => {
                assert_eq!(format, RasterFormat::Png);
                assert!(data.starts_with(b"\x89PNG\r\n\x1a\n"));
            }
            _ => panic!("Expected Raster output"),
        }
        assert_eq!(qr_result.width, 200);
        assert_eq!(qr_result.height, 200);

        // Test JPEG rendering
        let jpeg_result = render_qr_code(
            "https://jedi.org",
            None,
            RasterFormat::Jpeg,
            200,
        );
        assert!(jpeg_result.is_ok());
        let qr_result = jpeg_result.unwrap();
        match qr_result.data {
            QrCodeOutput::Raster(data, format) => {
                assert_eq!(format, RasterFormat::Jpeg);
                assert!(data.starts_with(b"\xff\xd8"));
            }
            _ => panic!("Expected Raster output"),
        }
        assert_eq!(qr_result.width, 200);
        assert_eq!(qr_result.height, 200);
    }

    #[test]
    fn renders_qr_code_as_svg() {
        let result = render_qr_code_svg("https://jedi.org", None);
        match result.data {
            QrCodeOutput::Svg(svg) => {
                assert!(svg.starts_with("<svg"));
                assert!(svg.ends_with("</svg>\n"));
            }
            _ => panic!("Expected SVG output"),
        }
        // Size should be based on matrix dimensions (modules * 10 pixels)
        let matrix = generate_qr_matrix("https://jedi.org");
        let expected_size = (matrix.len() * 10) as u32;
        assert_eq!(result.width, expected_size);
        assert_eq!(result.height, expected_size);
    }
}
