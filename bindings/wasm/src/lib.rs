use wasm_bindgen::prelude::*;
use qrcode_core::{
    render_qr_code,
    render_qr_code_svg,
    generate_qr_matrix,
    RasterFormat,
    QrCodeOutput,
    QrRenderConfig,
    FinderShape,
    DataShape,
    FinderStyle,
    DataStyle
};
use base64::{engine::general_purpose, Engine as _};

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
pub struct QrConfig {
    finder_shape: String,
    data_shape: String,
    finder_color: String,
    data_color: String,
}

#[wasm_bindgen]
impl QrConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(finder_shape: String, data_shape: String, finder_color: String, data_color: String) -> Self {
        QrConfig {
            finder_shape,
            data_shape,
            finder_color,
            data_color,
        }
    }
}

fn convert_config(config: &QrConfig) -> QrRenderConfig {
    let finder_shape = match config.finder_shape.as_str() {
        "Square" => FinderShape::Square,
        "Dot" => FinderShape::Dot,
        "Rounded" => FinderShape::Rounded,
        "Triangle" => FinderShape::Triangle,
        _ => FinderShape::Square,
    };

    let data_shape = match config.data_shape.as_str() {
        "Square" => DataShape::Square,
        "Dot" => DataShape::Dot,
        "Rounded" => DataShape::Rounded,
        "Triangle" => DataShape::Triangle,
        _ => DataShape::Dot,
    };

    QrRenderConfig {
        finder_shape,
        data_shape,
        finder_styling: FinderStyle::Color(config.finder_color.clone()),
        data_styling: DataStyle::Color(config.data_color.clone()),
    }
}

/// Renders a QR code as SVG and returns the result as a string
#[wasm_bindgen]
pub fn render_qr_svg(url: &str, config: Option<QrConfig>) -> String {
    let qr_config = config.map(|c| convert_config(&c));
    let result = render_qr_code_svg(url, qr_config.as_ref());
    match result.data {
        QrCodeOutput::Svg(svg) => svg,
        _ => panic!("Expected SVG output"),
    }
}

/// Renders a QR code as PNG and returns the result as a Uint8Array
#[wasm_bindgen]
pub fn render_qr_png(url: &str, config: Option<QrConfig>) -> Result<String, JsValue> {
    let qr_config = config.map(|c| convert_config(&c));
    let matrix = generate_qr_matrix(url);
    let size = (matrix.len() * 10) as u32;
    let result = render_qr_code(url, qr_config.as_ref(), RasterFormat::Png, size)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    match result.data {
        QrCodeOutput::Raster(data, _) => Ok(general_purpose::STANDARD.encode(data)),
        _ => Err(JsValue::from_str("Expected raster output")),
    }
}

/// Renders a QR code as JPEG and returns the result as a Uint8Array
#[wasm_bindgen]
pub fn render_qr_jpeg(url: &str, config: Option<QrConfig>) -> Result<String, JsValue> {
    let qr_config = config.map(|c| convert_config(&c));
    let matrix = generate_qr_matrix(url);
    let size = (matrix.len() * 10) as u32;
    let result = render_qr_code(url, qr_config.as_ref(), RasterFormat::Jpeg, size)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    match result.data {
        QrCodeOutput::Raster(data, _) => Ok(general_purpose::STANDARD.encode(data)),
        _ => Err(JsValue::from_str("Expected raster output")),
    }
}

/// Returns the dimensions of a QR code for a given URL
#[wasm_bindgen]
pub fn get_qr_dimensions(url: &str, config: Option<QrConfig>) -> Vec<u32> {
    let qr_config = config.map(|c| convert_config(&c));
    let result = render_qr_code_svg(url, qr_config.as_ref());
    vec![result.width, result.height]
}
