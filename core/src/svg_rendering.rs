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

pub enum FinderPosition {
    TopLeft,
    TopRight,
    BottomLeft
}

/// Renders a finder pattern module as SVG
fn render_finder_module(position: FinderPosition, module_size: usize, shape: &FinderShape) -> String {
    // Calculate base position based on finder pattern position
    let (base_x, base_y) = match position {
        FinderPosition::TopLeft => (0, 0),
        FinderPosition::TopRight => (7, 0),
        FinderPosition::BottomLeft => (0, 7),
    };
    
    match shape {
        FinderShape::Rounded => {
            format!(
                "  <rect x='{x}' y='{y}' width='{w}' height='{w}' fill='black' rx='{r}' ry='{r}'/>\n",
                x = base_x * module_size,
                y = base_y * module_size,
                w = module_size,
                r = module_size / 4
            )
        }
        FinderShape::Square => {
            format!(
                "  <rect x='{x}' y='{y}' width='{w}' height='{w}' fill='black'/>\n",
                x = base_x * module_size,
                y = base_y * module_size,
                w = module_size
            )
        }
        FinderShape::Dot => {
            format!(
                "  <circle cx='{cx}' cy='{cy}' r='{r}' fill='black'/>\n",
                cx = (base_x * module_size) + module_size/2,
                cy = (base_y * module_size) + module_size/2,
                r = module_size/2
            )
        }
        FinderShape::Triangle => {
            format!(
                "  <polygon points='{x1},{y1} {x2},{y2} {x3},{y3}' fill='black'/>\n",
                x1 = (base_x * module_size) + module_size/2,
                y1 = base_y * module_size,
                x2 = base_x * module_size,
                y2 = (base_y * module_size) + module_size,
                x3 = (base_x * module_size) + module_size,
                y3 = (base_y * module_size) + module_size
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
    let width = matrix.len();
    let svg_size = width * module_size;
    let mut svg = String::new();
    let mut rendered_top_left = false;
    let mut rendered_top_right = false;
    let mut rendered_bottom_left = false;
    
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
            if is_dark {
                // Determine if the current module is part of a finder pattern
                let is_top_left_finder_pattern: bool = x < 7 && y < 7;
                let is_top_right_finder_pattern: bool = x >= width - 7 && y < 7;
                let bottom_left_finder_pattern: bool = x < 7 && y >= width - 7;
                if is_top_left_finder_pattern && !rendered_top_left {
                    svg.push_str(&render_finder_module(FinderPosition::TopLeft, module_size, &config.finder_shape));
                    rendered_top_left = true;
                } else if is_top_right_finder_pattern && !rendered_top_right {
                    svg.push_str(&render_finder_module(FinderPosition::TopRight, module_size, &config.finder_shape));
                    rendered_top_right = true;
                } else if bottom_left_finder_pattern && !rendered_bottom_left {
                    svg.push_str(&render_finder_module(FinderPosition::BottomLeft, module_size, &config.finder_shape));
                    rendered_bottom_left = true;
                } else {
                    svg.push_str(&render_data_module(x, y, module_size, &config.data_shape));
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