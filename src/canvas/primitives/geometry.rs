use crate::canvas::types::Point;

/// The pure geometric description of a primitive — no style, no transform.
/// Polygon is reserved for future use.
#[derive(Clone, Debug)]
pub enum Geometry {
    /// Freehand polyline (smoothed at render time with Catmull-Rom).
    Stroke(Vec<Point>),
    Line {
        start: Point,
        end: Point,
    },
    Arrow {
        start: Point,
        end: Point,
    },
    Rect {
        origin: Point,
        size: (f64, f64),
    },
    Oval {
        origin: Point,
        size: (f64, f64),
    },
    // TODO: Add polygon support
    #[allow(dead_code)]
    Polygon {
        points: Vec<Point>,
    },
    // TODO: Add image support
    #[allow(dead_code)]
    Image {
        origin: Point,
        size: (f64, f64),
        data_url: String,
    },
    // TODO: Add text support
    #[allow(dead_code)]
    Text {
        origin: Point,
        content: String,
    },
}

impl Geometry {
    pub fn is_empty(&self) -> bool {
        match self {
            Geometry::Stroke(pts) => pts.is_empty(),
            _ => false,
        }
    }
}
