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

/// Renders a QR code matrix as an SVG string.
/// Each module is rendered as a 10x10 pixel square.
pub fn render_qr_matrix_as_svg(matrix: &[Vec<bool>], user_defined_config: Option<&QrRenderConfig>) -> String {
    let default_config = QrRenderConfig::default();
    let config = user_defined_config.unwrap_or(&default_config);
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
            // Finder patterns are 7x7 squares in the top-left, top-right, and bottom-left corners.
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

                // Choose shape based on config and module type
                match (is_finder, &config.finder_shape, &config.data_shape) {
                    (true, FinderShape::Rounded, _) => {
                        // Render rounded rectangle for dark finder modules
                        // UNTESTED
                         svg.push_str(&format!(
                            "  <rect x='{x}' y='{y}' width='{w}' height='{w}' fill='black' rx='{r}' ry='{r}'/>\n",
                            x = px,
                            y = py,
                            w = module_size,
                            r = module_size / 4 // Radius for rounding
                        ));
                    }
                    (true, FinderShape::Square, _) => {
                        // Render square for dark finder modules
                        svg.push_str(&format!(
                            "  <rect x='{x}' y='{y}' width='{w}' height='{w}' fill='black'/>\n",
                            x = px,
                            y = py,
                            w = module_size
                        ));
                    }
                    (true, FinderShape::Dot, _) => {
                        // Render circle for dark finder modules
                        svg.push_str(&format!(
                            "  <circle cx='{cx}' cy='{cy}' r='{r}' fill='green'/>\n",
                            cx = px + module_size/2,
                            cy = py + module_size/2,
                            r = module_size/2
                        ));
                    }
                    (true, FinderShape::Triangle, _) => {
                        // UNTESTED
                        // Render triangle for dark finder modules
                        svg.push_str(&format!(
                            "  <polygon points='{x1},{y1} {x2},{y2} {x3},{y3}' fill='black'/>\n",
                            x1 = px + module_size/2,
                            y1 = py,
                            x2 = px,
                            y2 = py + module_size,
                            x3 = px + module_size,
                            y3 = py + module_size
                        ));
                    }
                    (false, _, DataShape::Rounded) => {
                        // UNTESTED
                        // Render rounded rectangle for dark data modules
                         svg.push_str(&format!(
                            "  <rect x='{x}' y='{y}' width='{w}' height='{w}' fill='black' rx='{r}' ry='{r}'/>\n",
                            x = px,
                            y = py,
                            w = module_size,
                            r = module_size / 4 // Radius for rounding
                        ));
                    }
                    (false, _, DataShape::Square) => {
                        // Render square for dark data modules
                         svg.push_str(&format!(
                            "  <rect x='{x}' y='{y}' width='{w}' height='{w}' fill='black'/>\n",
                            x = px,
                            y = py,
                            w = module_size
                        ));
                    }
                    (false, _, DataShape::Dot) => {
                        // Render circle for dark data modules
                        svg.push_str(&format!(
                            "  <circle cx='{cx}' cy='{cy}' r='{r}' fill='yellow'/>\n",
                            cx = px + module_size/2,
                            cy = py + module_size/2,
                            r = module_size/2
                        ));
                    }
                    (false, _, DataShape::Triangle) => {
                        // UNTESTED
                        // Render triangle for dark data modules
                        svg.push_str(&format!(
                            "  <polygon points='{x1},{y1} {x2},{y2} {x3},{y3}' fill='black'/>\n",
                            x1 = px + module_size/2,
                            y1 = py,
                            x2 = px,
                            y2 = py + module_size,
                            x3 = px + module_size,
                            y3 = py + module_size
                        ));
                    }
                }
            }
        }
    }
    svg.push_str("</svg>\n");
    svg
}

impl Default for QrRenderConfig {
    fn default() -> Self {
        QrRenderConfig {
            finder_shape: FinderShape::Dot,
            data_shape: DataShape::Dot,
        }
    }
}

#[cfg(test)]
mod tests {
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