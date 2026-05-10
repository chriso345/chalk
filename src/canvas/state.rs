use crate::canvas::{
    history::History,
    primitives::{Primitive, ShapeInProgress},
    styles::ChalkStyles,
    tool::Tool,
    types::{Point, ViewTransform},
};

/// What the user is currently drawing. Cleared on pointer-up.
#[derive(Clone, Debug)]
pub enum ActiveDrawing {
    Stroke(Vec<Point>),
    Shape(ShapeInProgress),
}

impl ActiveDrawing {
    /// Consume into a `Primitive` for committing to history, or `None` if empty.
    pub fn into_primitive(self) -> Option<Primitive> {
        match self {
            ActiveDrawing::Stroke(pts) if pts.is_empty() => None,
            ActiveDrawing::Stroke(pts) => Some(Primitive::Stroke(pts)),
            ActiveDrawing::Shape(s) => Some(Primitive::Shape(s.build())),
        }
    }

    /// Borrow as a preview `Primitive` without consuming.
    pub fn preview(&self) -> Option<Vec<Primitive>> {
        match self {
            ActiveDrawing::Stroke(pts) => Some(vec![Primitive::Stroke(pts.clone())]),
            ActiveDrawing::Shape(s) => Some(vec![Primitive::Shape(s.build())]),
        }
    }
}

/// The entire state of the whiteboard.
pub struct WhiteboardState {
    /// The history of committed strokes and undo/redo state.
    pub history: History,
    /// The stroke currently being drawn, if any. Not yet part of history.
    pub active: Option<ActiveDrawing>,
    /// Whether the user is currently drawing (pointer down).
    pub is_drawing: bool,

    /// The current view transform (pan and zoom).
    pub vt: ViewTransform,

    /// The currently selected tool.
    pub tool: Tool,

    pub style: ChalkStyles<'static>,

    // TODO: This should be scaled with zoom to keep it visually consistent.
    /// Line width in world units (not pixels, so it scales with zoom).
    pub stroke_width: f64,

    /// Screen-space position of the last pointer event, used by Pan to
    /// compute deltas.
    pub last_pan_pos: Option<Point>,
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
            active: None,
            is_drawing: false,
            vt: ViewTransform::default(),
            tool: Tool::default(),

            style: ChalkStyles::dark(),
            stroke_width: 2.0,

            last_pan_pos: None,
        }
    }

    pub fn set_tool(&mut self, tool: Tool) {
        // Cancel any in-progress drawing when switching tools.
        self.active = None;
        self.is_drawing = false;
        self.last_pan_pos = None;
        self.tool = tool;
    }

    pub fn begin_drawing(&mut self, screen: Point, is_middle_mouse: bool) {
        let world = self.vt.screen_to_world(screen.0, screen.1);
        self.is_drawing = true;
        if is_middle_mouse {
            self.last_pan_pos = Some(screen);
            self.active = None;
            return;
        }

        self.active = match self.tool {
            Tool::Pan => {
                self.last_pan_pos = Some(screen);
                None
            }
            Tool::Pointer => None,
            Tool::Pen => Some(ActiveDrawing::Stroke(vec![world])),
            Tool::Shape(kind) => Some(ActiveDrawing::Shape(ShapeInProgress::new(kind, world))),
        };
    }

    pub fn update_drawing(&mut self, screen: Point, snap: bool, is_middle_mouse: bool) {
        if is_middle_mouse || self.tool == Tool::Pan {
            if let Some((lx, ly)) = self.last_pan_pos {
                self.vt.offset_x += screen.0 - lx;
                self.vt.offset_y += screen.1 - ly;
            }
            self.last_pan_pos = Some(screen);
            return;
        }

        let world = self.vt.screen_to_world(screen.0, screen.1);
        match &mut self.active {
            Some(ActiveDrawing::Stroke(pts)) => pts.push(world),
            Some(ActiveDrawing::Shape(s)) => s.update(world, snap),
            None => {}
        }
    }

    /// End drawing and return the finished primitive (if any) to commit.
    pub fn end_drawing(&mut self) -> Option<Primitive> {
        self.is_drawing = false;
        self.last_pan_pos = None;
        self.active.take().and_then(|a| a.into_primitive())
    }

    pub fn undo(&mut self) {
        self.history.undo();
    }

    pub fn redo(&mut self) {
        self.history.redo();
    }

    pub fn clear(&mut self) {
        self.history.clear();
        self.active = None;
    }

    pub fn change_theme(&mut self, to_light: bool) {
        self.style = if to_light {
            ChalkStyles::light()
        } else {
            ChalkStyles::dark()
        }
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
