use serde::{Deserialize, Serialize};

pub mod geometry;
pub mod handle;
pub mod renderer;
pub mod shape;
pub mod style;
pub mod transform;

pub mod collections;

pub use geometry::Geometry;
pub use shape::{ShapeInProgress, ShapeKind};
pub use style::PrimitiveStyle;
pub use transform::Transform;

/// A fully self-contained drawable unit.
/// Carries its own geometry, style, and transform.
/// Groups / compound objects are handled at a higher layer (see `Element`).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Primitive {
    pub geometry: Geometry,
    pub style: PrimitiveStyle,
    pub transform: Transform,
}

impl Primitive {
    pub fn new(geometry: Geometry, style: PrimitiveStyle) -> Self {
        Self {
            geometry,
            style,
            transform: Transform::default(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.geometry.is_empty()
    }

    /// Returns the handle positions for this primitive.
    pub fn handle_positions(&self) -> Vec<(f64, f64)> {
        match &self.geometry {
            crate::canvas::primitives::geometry::Geometry::Line { start, end }
            | crate::canvas::primitives::geometry::Geometry::Arrow { start, end } => {
                let (tx, ty) = self.transform.position;
                vec![(start.0 + tx, start.1 + ty), (end.0 + tx, end.1 + ty)]
            }
            _ => {
                // Use bounding box handles for other types
                let (tx, ty) = self.transform.position;
                if let Some((minx, miny, maxx, maxy)) = self.geometry.aabb() {
                    crate::canvas::primitives::handle::HandleKind::ALL
                        .iter()
                        .map(|kind| kind.position(minx + tx, miny + ty, maxx + tx, maxy + ty))
                        .collect()
                } else {
                    vec![]
                }
            }
        }
    }
}
