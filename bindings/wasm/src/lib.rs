use wasm_bindgen::prelude::*;
use qrcode_core::{generate_qr_matrix, render_qr_matrix_as_svg, QrRenderConfig};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[wasm_bindgen]
pub fn hello() -> String {
    qrcode_core::hello()
}

#[wasm_bindgen]
pub fn generate_qr_svg(url: &str) -> String {
    let matrix = generate_qr_matrix(url);
    let config = QrRenderConfig::default();
    render_qr_matrix_as_svg(&matrix, Some(&config))
}
