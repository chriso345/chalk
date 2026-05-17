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
#[derive(Clone, Debug)]
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
}
