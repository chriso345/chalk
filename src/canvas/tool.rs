use crate::canvas::primitives::ShapeKind;

/// The active tool - determines how pointer events are interpreted.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tool {
    /// Pans the viewport; no marks are made.
    Pan,
    /// Selecting pointer
    Pointer,
    /// Freehand pen stroke.
    Pen,
    /// Geometric shape; the specific kind is carried as `ShapeKind`.
    Shape(ShapeKind),
}

impl Tool {
    pub fn label(self) -> &'static str {
        match self {
            Tool::Pan => "pan",
            Tool::Pointer => "pointer",
            Tool::Pen => "pen",
            Tool::Shape(ShapeKind::Line) => "line",
            Tool::Shape(ShapeKind::Arrow) => "arrow",
            Tool::Shape(ShapeKind::Rect) => "rect",
            Tool::Shape(ShapeKind::Oval) => "circle",
        }
    }

    pub fn cursor(self) -> &'static str {
        match self {
            Tool::Pan => "grab",
            Tool::Pointer => "default",
            _ => "crosshair",
        }
    }
}

impl Default for Tool {
    fn default() -> Self {
        Tool::Pointer
    }
}
