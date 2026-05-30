use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HandleKind {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
    PivotStart,
    PivotEnd,
}

impl HandleKind {
    // PivotStart and PivotEnd Handles are excluded from this list as they are only used for line endpoints.
    pub const ALL: &'static [HandleKind] = &[
        HandleKind::TopLeft,
        HandleKind::Top,
        HandleKind::TopRight,
        HandleKind::Left,
        HandleKind::Right,
        HandleKind::BottomLeft,
        HandleKind::Bottom,
        HandleKind::BottomRight,
    ];

    /// Returns the (x, y) center of this handle in world space given the AABB.
    pub fn position(&self, minx: f64, miny: f64, maxx: f64, maxy: f64) -> (f64, f64) {
        let n = 6.0;

        let mx = (minx + maxx) / 2.0;
        let my = (miny + maxy) / 2.0;

        match self {
            HandleKind::TopLeft => (minx - n, miny - n),
            HandleKind::Top => (mx, miny - n),
            HandleKind::TopRight => (maxx + n, miny - n),

            HandleKind::Left => (minx - n, my),
            HandleKind::Right => (maxx + n, my),

            HandleKind::BottomLeft => (minx - n, maxy + n),
            HandleKind::Bottom => (mx, maxy + n),
            HandleKind::BottomRight => (maxx + n, maxy + n),

            // TODO: Fix positioning
            HandleKind::PivotStart => (minx, miny),
            HandleKind::PivotEnd => (maxx, maxy),
        }
    }

    /// Which axes this handle moves. (moves_x, moves_min_x, moves_y, moves_min_y)
    pub fn axes(&self) -> (bool, bool, bool, bool) {
        match self {
            HandleKind::TopLeft => (true, true, true, true),
            HandleKind::Top => (false, false, true, true),
            HandleKind::TopRight => (true, false, true, true),
            HandleKind::Left => (true, true, false, false),
            HandleKind::Right => (true, false, false, false),
            HandleKind::BottomLeft => (true, true, true, false),
            HandleKind::Bottom => (false, false, true, false),
            HandleKind::BottomRight => (true, false, true, false),
            HandleKind::PivotStart => (true, true, true, true),
            HandleKind::PivotEnd => (true, true, true, true),
        }
    }

    pub fn cursor(&self) -> &'static str {
        match self {
            HandleKind::TopLeft | HandleKind::BottomRight => "nwse-resize",
            HandleKind::TopRight | HandleKind::BottomLeft => "nesw-resize",
            HandleKind::Top | HandleKind::Bottom => "ns-resize",
            HandleKind::Left | HandleKind::Right => "ew-resize",
            HandleKind::PivotStart | HandleKind::PivotEnd => "move",
        }
    }
}
