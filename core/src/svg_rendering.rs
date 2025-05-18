/// configuration for QR code rendering
pub struct QrRenderConfig {
    pub finder_shape: FinderShape,
    pub data_shape: DataShape
}

pub enum FinderShape {
    Square,
    Dot,
    Rounded,
    Triangle
}

pub enum DataShape {
    Square,
    Dot,
    Rounded,
    Triangle
}

/// Renders a finder pattern module as SVG
fn render_finder_module(x_px: usize, y_px: usize, module_size: usize, shape: &FinderShape) -> String {
    // Create a group for the finder pattern
    match shape {
        FinderShape::Square | FinderShape::Rounded | FinderShape::Dot | FinderShape::Triangle => {
            format!(
                r#"  <g transform="translate({x}, {y})">
                        <rect width="{size}" height="{size}" fill="rebeccapurple"/>
                    </g>
                "#,
                x = x_px,
                y = y_px,
                size = 7 * module_size // The finder pattern is 7x7 modules
            )
        }
    }
}

/// Renders a data module as SVG
fn render_data_module(x: usize, y: usize, module_size: usize, shape: &DataShape) -> String {
    let px = x * module_size;
    let py = y * module_size;
    
    match shape {
        DataShape::Rounded => {
            format!(
                "  <rect x='{x}' y='{y}' width='{w}' height='{w}' fill='black' rx='{r}' ry='{r}'/>\n",
                x = px,
                y = py,
                w = module_size,
                r = module_size / 4
            )
        }
        DataShape::Square => {
            format!(
                "  <rect x='{x}' y='{y}' width='{w}' height='{w}' fill='black'/>\n",
                x = px,
                y = py,
                w = module_size
            )
        }
        DataShape::Dot => {
            format!(
                "  <circle cx='{cx}' cy='{cy}' r='{r}' fill='black'/>\n",
                cx = px + module_size/2,
                cy = py + module_size/2,
                r = module_size/2
            )
        }
        DataShape::Triangle => {
            format!(
                "  <polygon points='{x1},{y1} {x2},{y2} {x3},{y3}' fill='black'/>\n",
                x1 = px + module_size/2,
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
pub fn render_qr_matrix_as_svg(matrix: &[Vec<bool>], user_defined_config: Option<&QrRenderConfig>) -> String {
    let default_config = QrRenderConfig::default();
    let config = user_defined_config.unwrap_or(&default_config);
    let module_size = 10; // pixels per module
    let width = matrix.len(); // width in modules
    let svg_size = width * module_size; // total size in pixels
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
    svg.push_str(&render_finder_module(0, 0, module_size, &config.finder_shape)); // Top-left
    svg.push_str(&render_finder_module(svg_size - finder_pattern_size_px, 0, module_size, &config.finder_shape)); // Top-right
    svg.push_str(&render_finder_module(0, svg_size - finder_pattern_size_px, module_size, &config.finder_shape)); // Bottom-left
    
    // Draw data modules
    for (y, row) in matrix.iter().enumerate() {
        for (x, &is_dark) in row.iter().enumerate() {
            // Determine if the current module is part of a finder pattern
            let is_finder = 
                (x < 7 && y < 7) ||
                (x >= width - 7 && y < 7) ||
                (x < 7 && y >= width - 7);

            // Only render dark data modules
            if is_dark && !is_finder {
                 svg.push_str(&render_data_module(x, y, module_size, &config.data_shape));
            }
        }
    }
    
    svg.push_str("</svg>\n");
    svg
}

impl Default for QrRenderConfig {
    fn default() -> Self {
        QrRenderConfig {
            finder_shape: FinderShape::Square,
            data_shape: DataShape::Dot,
        }
    }
}

#[cfg(test)] mod tests {
    use crate::svg_rendering::{QrRenderConfig, render_qr_matrix_as_svg};
    use crate::generate_qr_matrix;

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