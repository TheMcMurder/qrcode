use log::info;
use console_log;
use std::sync::Once;

static INIT_LOGGER: Once = Once::new();
/// configuration for QR code rendering
pub struct QrRenderConfig {
    pub finder_shape: FinderShape,
    pub data_shape: DataShape,
    pub finder_styling: FinderStyle,
    pub data_styling: DataStyle
}

pub enum DataStyle {
    /// Color specification for the finder pattern
    /// Can be any valid SVG color (named color, hex code, or rgb value)
    Color(String)
}

pub enum FinderStyle {
    /// Color specification for the finder pattern
    /// Can be any valid SVG color (named color, hex code, or rgb value)
    Color(String)
}

pub enum FinderShape {
    Square,
    Dot,
    Rounded,
    Triangle,
}

pub enum DataShape {
    Square,
    Dot,
    Rounded,
    Triangle,
}

/// Renders a finder pattern module as SVG
fn render_finder_module(
    x_px: usize,
    y_px: usize,
    module_size: usize,
    shape: &FinderShape,
    style: &FinderStyle,
) -> String {
    // Get the color from the style
    let color = match style {
        FinderStyle::Color(c) => c,
    };
    
    // Create a group for the finder pattern
    match shape {
        // The standard finder pattern shape is the same regardless of the configured FinderShape
        FinderShape::Square | FinderShape::Rounded | FinderShape::Triangle => {
            format!(
                r#"
                    <g transform="translate({x}, {y})">
                        <rect width="{outer_bar_width}" height="{outer_bar_thickness}" fill="{color}"/>
                        <rect y="{bottom_bar_y}" width="{outer_bar_width}" height="{outer_bar_thickness}" fill="{color}"/>
                        <rect width="{outer_bar_thickness}" height="{outer_bar_height}" fill="{color}" x="{left_bar_x}" y="{left_bar_y}"/>
                        <rect width="{outer_bar_thickness}" height="{outer_bar_height}" fill="{color}" x="{right_bar_x}" y="{right_bar_y}"/>
                        <rect x="{inner_pos}" y="{inner_pos}" width="{inner_size}" height="{inner_size}" fill="{color}"/>
                    </g>
                "#,
                x = x_px,
                y = y_px,
                color = color,
                outer_bar_thickness = module_size, // 1 module thickness
                outer_bar_width = 7 * module_size, // 7 modules wide
                outer_bar_height = 5 * module_size, // 5 modules tall (7 total - 1 top - 1 bottom)
                bottom_bar_y = 6 * module_size,    // Starts at module row 6
                left_bar_x = 0,                    // Starts at module col 0
                left_bar_y = 1 * module_size,      // Starts at module row 1
                right_bar_x = 6 * module_size,     // Starts at module col 6
                right_bar_y = 1 * module_size,     // Starts at module row 1
                inner_size = 3 * module_size,      // 3x3 modules for the innermost black square
                inner_pos = 2 * module_size        // Offset by 2 modules
            )
        }
        FinderShape::Dot => {
            let center = 3.5 * module_size as f32;
            let ring_radius = 3.0 * module_size as f32;
            let ring_stroke_width = 1.0 * module_size as f32;
            let r_dot = 1.5 * module_size as f32;

            format!(
                r#"
                    <g transform="translate({x}, {y})">
                        <circle cx='{center}' cy='{center}' r='{ring_radius}' fill='none' stroke='{color}' stroke-width='{ring_stroke_width}'/>
                        <circle cx='{center}' cy='{center}' r='{r_dot}' fill='{color}'/>
                    </g>
                "#,
                x = x_px,
                y = y_px,
                center = center,
                ring_radius = ring_radius,
                ring_stroke_width = ring_stroke_width,
                r_dot = r_dot,
                color = color,
            )
        }
    }
}

/// Renders a data module as SVG
fn render_data_module(x: usize, y: usize, module_size: usize, shape: &DataShape, style: &DataStyle) -> String {
    let px = x * module_size;
    let py = y * module_size;
    // Get the color from the style
    let color = match style {
        DataStyle::Color(c) => c,
    };

    match shape {
        DataShape::Rounded => {
            format!(
                "  <rect x='{x}' y='{y}' width='{w}' height='{w}' fill='{color}' rx='{r}' ry='{r}'/>\n",
                x = px,
                y = py,
                w = module_size,
                r = module_size / 4,
                color = color
            )
        }
        DataShape::Square => {
            format!(
                "  <rect x='{x}' y='{y}' width='{w}' height='{w}' fill='{color}'/>\n",
                x = px,
                y = py,
                w = module_size,
                color = color
            )
        }
        DataShape::Dot => {
            format!(
                "  <circle cx='{cx}' cy='{cy}' r='{r}' fill='{color}'/>\n",
                cx = px + module_size / 2,
                cy = py + module_size / 2,
                r = module_size / 2,
                color = color
            )
        }
        DataShape::Triangle => {
            format!(
                "  <polygon points='{x1},{y1} {x2},{y2} {x3},{y3}' fill='black'/>\n",
                x1 = px + module_size / 2,
                y1 = py,
                x2 = px,
                y2 = py + module_size,
                x3 = px + module_size,
                y3 = py + module_size
            )
        }
    }
}

/// Renders a QR code matrix as an SVG string.
/// Each module is rendered as a 10x10 pixel square.
pub fn render_qr_matrix_as_svg(
    matrix: &[Vec<bool>],
    user_defined_config: Option<&QrRenderConfig>,
) -> String {
    // Initialize the console logger only once
    INIT_LOGGER.call_once(|| {
        console_log::init_with_level(log::Level::Info).expect("Failed to initialize console logger");
    });
    
    info!("Starting QR code SVG rendering");
    let default_config = QrRenderConfig::default();
    let config = user_defined_config.unwrap_or(&default_config);
    let module_size = 10; // pixels per module
    let width = matrix.len(); // width in modules
    let svg_size = width * module_size; // total size in pixels
    info!("Rendering QR code with size {}x{} modules ({}x{} pixels)", width, width, svg_size, svg_size);
    
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

    // Render finder patterns (7x7 modules each)
    let finder_pattern_size_px = 7 * module_size;
    svg.push_str(&render_finder_module(
        0,
        0,
        module_size,
        &config.finder_shape,
        &config.finder_styling,
    )); // Top-left
    svg.push_str(&render_finder_module(
        svg_size - finder_pattern_size_px,
        0,
        module_size,
        &config.finder_shape,
        &config.finder_styling,
    )); // Top-right
    svg.push_str(&render_finder_module(
        0,
        svg_size - finder_pattern_size_px,
        module_size,
        &config.finder_shape,
        &config.finder_styling,
    )); // Bottom-left

    // Draw data modules
    for (y, row) in matrix.iter().enumerate() {
        for (x, &is_dark) in row.iter().enumerate() {
            // Determine if the current module is part of a finder pattern
            let is_finder =
                (x < 7 && y < 7) || (x >= width - 7 && y < 7) || (x < 7 && y >= width - 7);

            // Only render dark data modules
            if is_dark && !is_finder {
                svg.push_str(&render_data_module(x, y, module_size, &config.data_shape, &config.data_styling));
            }
        }
    }

    svg.push_str("</svg>\n");
    info!("Completed QR code SVG rendering");
    svg
}

impl Default for QrRenderConfig {
    fn default() -> Self {
        QrRenderConfig {
            finder_shape: FinderShape::Square,
            data_shape: DataShape::Dot,
            finder_styling: FinderStyle::Color("green".to_string()),
            data_styling: DataStyle::Color(("red").to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::generate_qr_matrix;
    use crate::svg_rendering::{render_qr_matrix_as_svg, QrRenderConfig};

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
        assert!(svg.ends_with("</svg>\n"));
    }
}
