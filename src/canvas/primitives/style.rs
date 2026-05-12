/// All styling properties for a single primitive.
/// Colors are stored as CSS color strings (cheap to clone, easy to pass to Canvas2D).
#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveStyle {
    /// Stroke color. `None` means no stroke is drawn.
    pub stroke_color: Option<String>,
    /// Stroke width
    pub stroke_width: f64,
    /// Fill color. `None` means no fill is drawn.
    pub fill_color: Option<String>,
}

impl PrimitiveStyle {
    pub fn new(stroke_color: impl Into<String>, stroke_width: f64) -> Self {
        Self {
            stroke_color: Some(stroke_color.into()),
            stroke_width,
            fill_color: None,
        }
    }

    pub fn with_fill(mut self, fill: impl Into<String>) -> Self {
        self.fill_color = Some(fill.into());
        self
    }

    pub fn with_no_stroke(mut self) -> Self {
        self.stroke_color = None;
        self
    }
}

impl Default for PrimitiveStyle {
    fn default() -> Self {
        Self {
            stroke_color: Some("#F2F0EF".to_string()),
            stroke_width: 2.0,
            fill_color: None,
        }
    }
}
