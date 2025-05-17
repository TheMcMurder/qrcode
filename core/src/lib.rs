pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// configuration for QR code rendering
pub struct QrRenderConfig {
    pub finder_shape: FinderShape,
    pub data_shape: DataShape
}

pub enum FinderShape {
    Square,
    Circle,
    Rounded,
    Triangle
}

pub enum DataShape {
    Square,
    Circle,
    Rounded,
    Triange
}

pub fn hello() -> String {
    "Hello from Rust!".to_string()
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

/// Renders a QR code matrix as an SVG string.
/// Each module is rendered as a 10x10 pixel square.
pub fn render_qr_matrix_as_svg(matrix: &[Vec<bool>], _user_defined_config: Option<&QrRenderConfig>) -> String {
    // let config= user_defined_config.unwrap_or(&QrRenderConfig::default());
    let module_size = 10; // pixels per module
    let width = matrix.len();
    let svg_size = (width * module_size) as u32;
    let mut svg = String::new();
    // SVG header
    svg.push_str(&format!(
        r#"<svg xmlns='http://www.w3.org/2000/svg' width='{size}' height='{size}' viewBox='0 0 {size} {size}' shape-rendering='crispEdges'>\n"#,
        size = svg_size
    ));
    // White background
    svg.push_str(&format!(
        r#"  <rect width='{size}' height='{size}' fill='white'/>\n"#,
        size = svg_size
    ));
    // Draw modules
    for (y, row) in matrix.iter().enumerate() {
        for (x, &is_dark) in row.iter().enumerate() {
            // Determine if the current module is part of a finder pattern
            let is_finder = 
                // Top-left finder pattern
                (x < 7 && y < 7) ||
                // Top-right finder pattern
                (x >= width - 7 && y < 7) ||
                // Bottom-left finder pattern
                (x < 7 && y >= width - 7)
            ;

            if is_dark {
                let px = x * module_size;
                let py = y * module_size;
                // Use a different fill color for finder patterns for now
                let fill_color = if is_finder { "blue" } else { "black" };
                svg.push_str(&format!(
                    "  <rect x='{x}' y='{y}' width='{w}' height='{w}' fill='{fill}'/>\n",
                    x = px,
                    y = py,
                    w = module_size,
                    fill = fill_color
                ));
            }
        }
    }
    svg.push_str("</svg>\n");
    svg
}

impl Default for QrRenderConfig {
    fn default() -> Self {
        QrRenderConfig {
            finder_shape: FinderShape::Rounded,
            data_shape: DataShape::Rounded,
        }
    }
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
    fn renders_svg() {
        let matrix = generate_qr_matrix("https://sith.org");
        // Pass a default config to the rendering function
        let config = QrRenderConfig::default();
        let svg = render_qr_matrix_as_svg(&matrix, Some(&config));
        // Basic checks: SVG header and some black squares
        assert!(svg.starts_with("<svg"));
        // Check for both black (data) and blue (finder) modules
        assert!(svg.contains("fill='black'"));
        assert!(svg.contains("fill='blue'"));
        assert!(svg.ends_with("</svg>\n"));
    }
}
