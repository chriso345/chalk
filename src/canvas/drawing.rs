use crate::canvas::{
    primitives::{Geometry, Primitive, PrimitiveStyle, ShapeInProgress},
    types::Point,
};

/// What the user is currently drawing. Cleared on pointer-up.
#[derive(Clone, Debug)]
pub enum ActiveDrawing {
    Stroke(Vec<Point>),
    Shape(ShapeInProgress),
}

impl ActiveDrawing {
    /// Preview as primitives for rendering. Style is provided externally.
    pub fn preview(&self, style: &PrimitiveStyle) -> Option<Vec<Primitive>> {
        let geom = match self {
            ActiveDrawing::Stroke(pts) if pts.is_empty() => return None,
            ActiveDrawing::Stroke(pts) => Geometry::Stroke(pts.clone()),
            ActiveDrawing::Shape(s) => Geometry::from(s.build()),
        };
        Some(vec![Primitive::new(geom, style.clone())])
    }

    /// Consume into a committed `Primitive`, stamping the provided style.
    pub fn into_primitive(self, style: PrimitiveStyle) -> Option<Primitive> {
        let geom = match self {
            ActiveDrawing::Stroke(pts) if pts.is_empty() => return None,
            ActiveDrawing::Stroke(pts) => Geometry::Stroke(pts),
            ActiveDrawing::Shape(s) => s.build(),
        };
        Some(Primitive::new(geom, style))
    }
}
