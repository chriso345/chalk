use crate::canvas::{
    history::History,
    types::{Stroke, ViewTransform},
};

const DARK_MODE_BG_COLOR: &str = "#1A1A18";
const LIGHT_MODE_BG_COLOR: &str = "#F2F0EF";

const STROKE_COLOR_LIGHT: &str = "#1A1A18";
const STROKE_COLOR_DARK: &str = "#F2F0EF";

/// The entire state of the whiteboard.
pub struct WhiteboardState {
    /// The history of committed strokes and undo/redo state.
    pub history: History,
    /// The stroke currently being drawn, if any. Not yet part of history.
    pub active_stroke: Option<Stroke>,
    /// Whether the user is currently drawing (pointer down).
    pub is_drawing: bool,

    /// The current view transform (pan and zoom).
    pub vt: ViewTransform,

    /// Background colour - switches with dark mode.
    pub bg_color: &'static str,
    /// Line colour - currently only used for light mode
    pub stroke_color: &'static str,

    // TODO: This should be scaled with zoom to keep it visually consistent.
    /// Line width in world units (not pixels, so it scales with zoom).
    pub stroke_width: f64,
}

impl Default for WhiteboardState {
    fn default() -> Self {
        Self::new()
    }
}

impl WhiteboardState {
    pub fn new() -> Self {
        Self {
            history: History::default(),
            active_stroke: None,
            is_drawing: false,
            vt: ViewTransform::default(),

            bg_color: LIGHT_MODE_BG_COLOR,
            stroke_color: STROKE_COLOR_LIGHT,

            stroke_width: 2.0,
        }
    }

    pub fn begin_stroke(&mut self, world: (f64, f64)) {
        self.is_drawing = true;
        self.active_stroke = Some(vec![world]);
    }

    pub fn extend_stroke(&mut self, world: (f64, f64)) {
        if let Some(s) = &mut self.active_stroke {
            s.push(world);
        }
    }

    pub fn end_stroke(&mut self) -> Option<Stroke> {
        self.is_drawing = false;
        self.active_stroke.take().filter(|s| !s.is_empty())
    }

    pub fn undo(&mut self) {
        self.history.undo();
    }

    pub fn redo(&mut self) {
        self.history.redo();
    }

    pub fn clear(&mut self) {
        self.history.clear();
        self.active_stroke = None;
    }

    pub fn change_theme(&mut self, dark: bool) {
        self.bg_color = if dark {
            DARK_MODE_BG_COLOR
        } else {
            LIGHT_MODE_BG_COLOR
        };

        self.stroke_color = if dark {
            STROKE_COLOR_DARK
        } else {
            STROKE_COLOR_LIGHT
        };
    }

    pub fn set_zoom_centered(
        &mut self,
        target_zoom: f64,
        center_x: f64,
        center_y: f64,
        min: f64,
        max: f64,
    ) {
        let target_zoom = target_zoom.clamp(min, max);
        if (self.vt.zoom - target_zoom).abs() < f64::EPSILON {
            return;
        }
        let factor = target_zoom / self.vt.zoom;
        self.vt = self.vt.zoom_towards(center_x, center_y, factor, min, max);
    }
}
