use wasm_bindgen::prelude::*;
use qrcode_core::{
    render_qr_code,
    render_qr_code_svg,
    RasterFormat,
    QrCodeOutput
};

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

/// Renders a QR code as SVG and returns the result as a string
#[wasm_bindgen]
pub fn render_qr_svg(url: &str) -> String {
    let result = render_qr_code_svg(url, None);
    match result.data {
        QrCodeOutput::Svg(svg) => svg,
        _ => panic!("Expected SVG output"),
    }
}

/// Renders a QR code as PNG and returns the result as a Uint8Array
#[wasm_bindgen]
pub fn render_qr_png(url: &str, size: u32) -> Result<Vec<u8>, JsValue> {
    let result = render_qr_code(url, None, RasterFormat::Png, size)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    match result.data {
        QrCodeOutput::Raster(data, _) => Ok(data),
        _ => Err(JsValue::from_str("Expected raster output")),
    }
}

/// Renders a QR code as JPEG and returns the result as a Uint8Array
#[wasm_bindgen]
pub fn render_qr_jpeg(url: &str, size: u32) -> Result<Vec<u8>, JsValue> {
    let result = render_qr_code(url, None, RasterFormat::Jpeg, size)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    match result.data {
        QrCodeOutput::Raster(data, _) => Ok(data),
        _ => Err(JsValue::from_str("Expected raster output")),
    }
}

/// Returns the dimensions of a QR code for a given URL
#[wasm_bindgen]
pub fn get_qr_dimensions(url: &str) -> Vec<u32> {
    let result = render_qr_code_svg(url, None);
    vec![result.width, result.height]
}
