use serde::{Deserialize, Serialize};

/// A single (x, y) point in world space.
pub type Point = (f64, f64);

/// The current viewport transform: where the world origin sits on screen
/// and the zoom level.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ViewTransform {
    pub offset_x: f64,
    pub offset_y: f64,
    pub zoom: f64,
}

impl Default for ViewTransform {
    fn default() -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 0.0,
            zoom: 1.0,
        }
    }
}

impl ViewTransform {
    /// Convert a screen-space point to world-space.
    pub fn screen_to_world(&self, sx: f64, sy: f64) -> Point {
        (
            (sx - self.offset_x) / self.zoom,
            (sy - self.offset_y) / self.zoom,
        )
    }

    /// Zoom centred on a screen-space pivot point, clamped to [min, max].
    pub fn zoom_towards(
        &self,
        pivot_x: f64,
        pivot_y: f64,
        factor: f64,
        min: f64,
        max: f64,
    ) -> Self {
        let new_zoom = (self.zoom * factor).clamp(min, max);
        let ratio = new_zoom / self.zoom;
        Self {
            zoom: new_zoom,
            offset_x: pivot_x - (pivot_x - self.offset_x) * ratio,
            offset_y: pivot_y - (pivot_y - self.offset_y) * ratio,
        }
    }
}
