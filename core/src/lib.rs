pub fn add(left: u64, right: u64) -> u64 {
    left + right
}


pub fn hello() -> String {
    "Hello from Rust!".to_string()
}

mod svg_rendering;
pub use svg_rendering::{render_qr_matrix_as_svg, QrRenderConfig, FinderShape, DataShape};

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
}
