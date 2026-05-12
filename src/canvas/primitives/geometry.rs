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
    /// Not yet implemented; placeholder for future polygon support.
    #[allow(dead_code)]
    Polygon {
        points: Vec<Point>,
    },
}

impl Geometry {
    pub fn is_empty(&self) -> bool {
        match self {
            Geometry::Stroke(pts) => pts.is_empty(),
            // Geometry::Polygon(pts) => pts.is_empty(),
            _ => false,
        }
    }
}
