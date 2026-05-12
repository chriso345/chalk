use crate::canvas::types::Point;

/// Per-primitive spatial transform.
/// Currently only translation is operationally used;
/// rotation and scale are stored for future use.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    /// Translation in world units.
    pub position: Point,
    /// Rotation in radians (counter-clockwise). Not yet applied.
    pub rotation: f64,
    /// Uniform scale factor. Not yet applied.
    pub scale: f64,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: (0.0, 0.0),
            rotation: 0.0,
            scale: 1.0,
        }
    }
}
