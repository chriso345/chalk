pub mod renderer;
pub mod shape;

pub use shape::{Shape, ShapeInProgress, ShapeKind};

use crate::canvas::types::Point;

/// The atomic unit stored in history.
///
/// `Group` enables both hand-constructed groups and generator output to be
/// committed and undone as a single step.
#[derive(Clone, Debug)]
pub enum Primitive {
    Stroke(Vec<Point>),
    Shape(Shape),
    Group(Vec<Primitive>),
}

impl Primitive {
    pub fn is_empty(&self) -> bool {
        match self {
            Primitive::Stroke(pts) => pts.is_empty(),
            Primitive::Shape(_) => false,
            Primitive::Group(children) => children.is_empty(),
        }
    }
}
