use std::collections::{HashMap, HashSet};

use crate::{
    canvas::{
        action::ChalkAction,
        background::BackgroundKind,
        drawing::ActiveDrawing,
        history::History,
        primitives::{
            Geometry, Primitive, PrimitiveStyle, ShapeInProgress, geometry::primitives_aabb,
            handle::HandleKind,
        },
        styles::ChalkStyles,
        tool::Tool,
        types::{Point, ViewTransform},
    },
    ui::color::ChalkColor,
};

/// The entire state of the whiteboard.
pub struct WhiteboardState {
    /// The history of actions and undo/redo state.
    pub history: History,
    /// The stroke currently being drawn, if any. Not yet part of history.
    pub active: Option<ActiveDrawing>,
    /// Whether the user is currently drawing (pointer down).
    pub is_drawing: bool,

    /// The selected primitives that are being transformed, if any.
    pub selected: HashSet<usize>,
    /// The world-space position of the primitive's transforms when the drag began.
    pub drag_start_transforms: HashMap<usize, Point>,
    /// World-space pointer position when the drag began.
    pub drag_start_world: Option<Point>,

    /// If dragging a handle, which one and its initial state.
    pub drag_handle: Option<HandleKind>,
    /// The full AABB at the moment a handle drag began.
    pub drag_handle_initial_aabb: Option<(f64, f64, f64, f64)>,
    /// Per-primitive geometry snapshot at drag start, for proportional remap.
    pub drag_handle_initial_geoms: Vec<(usize, Geometry, Point)>, // (idx, geom, transform_pos)

    /// Primitives drawn to the canvas
    pub document: Vec<Primitive>,

    /// The current view transform (pan and zoom).
    pub vt: ViewTransform,

    /// The currently selected tool.
    pub tool: Tool,

    pub style: ChalkStyles<'static>,
    pub current_style: PrimitiveStyle,

    /// Screen-space position of the last pointer event, used by Pan to
    /// compute deltas.
    pub last_pan_pos: Option<Point>,

    /// Does the background show dotted grid
    pub background_pattern: BackgroundKind,
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

            selected: HashSet::new(),
            drag_start_transforms: HashMap::new(),
            drag_start_world: None,

            drag_handle: None,
            drag_handle_initial_aabb: None,
            drag_handle_initial_geoms: Vec::new(),

            document: Vec::<Primitive>::new(),

            vt: ViewTransform::default(),
            tool: Tool::default(),

            style: ChalkStyles::dark(),
            current_style: PrimitiveStyle::new("#FF0000", 2.0),

            last_pan_pos: None,

            background_pattern: BackgroundKind::None,
        }
    }

    pub fn set_tool(&mut self, tool: Tool) {
        // Cancel any in-progress drawing when switching tools.
        self.active = None;
        self.is_drawing = false;
        self.last_pan_pos = None;
        self.selected.clear();
        self.drag_start_transforms.clear();
        self.drag_start_world = None;
        self.drag_handle = None;
        self.drag_handle_initial_aabb = None;
        self.drag_handle_initial_geoms.clear();
        self.tool = tool;
    }

    pub fn set_background_pattern(&mut self, pattern: BackgroundKind) {
        self.background_pattern = pattern;
    }

    pub fn set_stroke_color(&mut self, color: ChalkColor) {
        let hex = color.to_hex().to_string();
        self.current_style.stroke_color = Some(hex.clone());

        if self.selected.is_empty() {
            return;
        }

        let actions: Vec<ChalkAction> = self
            .selected
            .iter()
            .filter(|&&idx| self.document[idx].style.stroke_color.is_some())
            .map(|&idx| {
                let before = self.document[idx].clone();
                let mut after = before.clone();
                after.style.stroke_color = Some(hex.clone());
                ChalkAction::Transform {
                    before,
                    after,
                    index: idx,
                }
            })
            .collect();

        for action in &actions {
            if let ChalkAction::Transform { index, after, .. } = action {
                self.document[*index] = after.clone();
            }
        }

        if actions.is_empty() {
            return;
        }

        let action = if actions.len() == 1 {
            actions.into_iter().next().unwrap()
        } else {
            ChalkAction::Batch { actions }
        };

        self.history.push_without_apply(action);
    }

    pub fn set_stroke_width(&mut self, width: f64) {
        self.current_style.stroke_width = width;

        if self.selected.is_empty() {
            return;
        }

        let actions: Vec<ChalkAction> = self
            .selected
            .iter()
            .map(|&idx| {
                let before = self.document[idx].clone();
                let mut after = before.clone();
                after.style.stroke_width = width;
                ChalkAction::Transform {
                    before,
                    after,
                    index: idx,
                }
            })
            .collect();

        for action in &actions {
            if let ChalkAction::Transform { index, after, .. } = action {
                self.document[*index] = after.clone();
            }
        }

        if actions.is_empty() {
            return;
        }

        let action = if actions.len() == 1 {
            actions.into_iter().next().unwrap()
        } else {
            ChalkAction::Batch { actions }
        };

        self.history.push_without_apply(action);
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
            Some(ActiveDrawing::Stroke(pts)) => {
                let world = self.vt.screen_to_world(screen.0, screen.1);

                let zoom = self.vt.zoom;
                let min_screen_dist = 8.0; // Tweakable constant
                let min_world_dist2 = (min_screen_dist / zoom).powi(2);

                if let Some(last) = pts.last() {
                    let dx = world.0 - last.0;
                    let dy = world.1 - last.1;
                    let dist2 = dx * dx + dy * dy;

                    if dist2 < min_world_dist2 {
                        return;
                    }
                }

                pts.push(world);
            }
            Some(ActiveDrawing::Shape(s)) => s.update(world, snap),
            None => {}
        }
    }

    /// End drawing and return the finished primitive (if any) to commit.
    pub fn end_drawing(&mut self) -> Option<Primitive> {
        self.is_drawing = false;
        self.last_pan_pos = None;
        self.active
            .take()
            .and_then(|a| a.into_primitive(self.current_style.clone()))
    }

    pub fn undo(&mut self) {
        self.history.undo(&mut self.document);
    }

    pub fn redo(&mut self) {
        self.history.redo(&mut self.document);
    }

    pub fn clear(&mut self) {
        self.history.clear(&mut self.document);
        self.active = None;
    }

    pub fn delete_selected(&mut self) {
        if self.selected.is_empty() {
            return;
        }

        let mut indices: Vec<usize> = self.selected.iter().copied().collect();
        indices.sort_unstable_by(|a, b| b.cmp(a));

        let actions: Vec<ChalkAction> = indices
            .iter()
            .map(|&idx| ChalkAction::Delete {
                primitive: self.document[idx].clone(),
                index: idx,
            })
            .collect();

        let action = if actions.len() == 1 {
            actions.into_iter().next().unwrap()
        } else {
            ChalkAction::Batch { actions }
        };

        self.history.apply(&mut self.document, action);
        self.selected.clear();
    }

    pub fn change_theme(&mut self, to_light: bool) {
        if (to_light && self.style.is_light()) || (!to_light && self.style.is_dark()) {
            return;
        }

        self.style.toggle();
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

    /// Returns the index of the topmost primitive hit by the given world point.
    pub fn hit_test(&self, world: Point) -> Option<usize> {
        for (i, prim) in self.document.iter().enumerate().rev() {
            if primitive_hit(prim, world) {
                return Some(i);
            }
        }
        None
    }

    /// Begin a drag of the selected primitive.
    pub fn begin_drag(&mut self, screen: Point) {
        let world = self.vt.screen_to_world(screen.0, screen.1);
        self.is_drawing = true;
        self.drag_start_world = Some(world);
        self.drag_start_transforms = self
            .selected
            .iter()
            .map(|&idx| (idx, self.document[idx].transform.position))
            .collect();
    }

    /// Move the selected primitive while dragging. Does NOT record history.
    pub fn update_drag(&mut self, screen: Point) {
        let world = self.vt.screen_to_world(screen.0, screen.1);
        if let Some(start_world) = self.drag_start_world {
            let delta = (world.0 - start_world.0, world.1 - start_world.1);
            for (&idx, &start_pos) in &self.drag_start_transforms {
                self.document[idx].transform.position =
                    (start_pos.0 + delta.0, start_pos.1 + delta.1);
            }
        }
    }

    /// Finish drag, commit to history. Returns (index, before, after) if a move happened.
    pub fn end_drag(&mut self) -> Vec<(usize, Point, Point)> {
        self.is_drawing = false;
        self.last_pan_pos = None;
        let mut moves = Vec::new();
        for (&idx, &start_pos) in &self.drag_start_transforms {
            let after = self.document[idx].transform.position;
            if (after.0 - start_pos.0).abs() > f64::EPSILON
                || (after.1 - start_pos.1).abs() > f64::EPSILON
            {
                moves.push((idx, start_pos, after));
            }
        }
        self.drag_start_transforms.clear();
        self.drag_start_world = None;
        moves
    }

    pub fn apply_selection(&mut self, world: Point, ctrl: bool) -> bool {
        let hit = self.hit_test(world);
        if ctrl {
            match hit {
                Some(idx) => {
                    if self.selected.contains(&idx) {
                        self.selected.remove(&idx);
                    } else {
                        self.selected.insert(idx);
                    }
                }
                None => {}
            }
        } else {
            let inside_selection = hit.map_or(false, |idx| self.selected.contains(&idx))
                || self.point_in_selection_bounds(world); // CHANGED
            if !inside_selection {
                self.selected.clear();
                if let Some(idx) = hit {
                    self.selected.insert(idx);
                }
            }
        }
        !self.selected.is_empty()
    }

    fn selection_bounds(&self) -> Option<(f64, f64, f64, f64)> {
        let prims = self.selected.iter().map(|&i| &self.document[i]);
        primitives_aabb(prims)
    }

    fn point_in_selection_bounds(&self, world: Point) -> bool {
        let Some((minx, miny, maxx, maxy)) = self.selection_bounds() else {
            return false;
        };
        const PAD: f64 = 6.0; // match the visual padding in the renderer
        world.0 >= minx - PAD
            && world.0 <= maxx + PAD
            && world.1 >= miny - PAD
            && world.1 <= maxy + PAD
    }

    /// Returns the handle kind and its world position if the point is within
    /// hit radius of any handle.
    pub fn hit_test_handle(&self, world: Point, zoom: f64) -> Option<HandleKind> {
        let (minx, miny, maxx, maxy) = self.selection_bounds()?;
        let hit_radius = 8.0 / zoom; // fixed screen-space radius
        for &kind in HandleKind::ALL {
            let (hx, hy) = kind.position(minx, miny, maxx, maxy);
            let dx = world.0 - hx;
            let dy = world.1 - hy;
            if dx * dx + dy * dy <= hit_radius * hit_radius {
                return Some(kind);
            }
        }
        None
    }

    pub fn begin_handle_drag(&mut self, screen: Point) {
        let world = self.vt.screen_to_world(screen.0, screen.1);
        self.is_drawing = true;
        self.drag_start_world = Some(world);
        self.drag_handle_initial_aabb = self.selection_bounds();
        self.drag_handle_initial_geoms = self
            .selected
            .iter()
            .map(|&idx| {
                (
                    idx,
                    self.document[idx].geometry.clone(),
                    self.document[idx].transform.position,
                )
            })
            .collect();
    }

    pub fn update_handle_drag(&mut self, screen: Point, snap: bool) {
        let world = self.vt.screen_to_world(screen.0, screen.1);
        let (Some(handle), Some(initial_aabb), Some(start_world)) = (
            self.drag_handle,
            self.drag_handle_initial_aabb,
            self.drag_start_world,
        ) else {
            return;
        };

        let delta = (world.0 - start_world.0, world.1 - start_world.1);
        let (mut minx, mut miny, mut maxx, mut maxy) = initial_aabb;

        let (ax, min_ax, ay, min_ay) = handle.axes();
        if ax {
            if min_ax {
                minx += delta.0;
            } else {
                maxx += delta.0;
            }
        }
        if ay {
            if min_ay {
                miny += delta.1;
            } else {
                maxy += delta.1;
            }
        }

        // Snap to square if shift held, using the larger axis delta.
        if snap {
            let dw = (maxx - minx) - (initial_aabb.2 - initial_aabb.0);
            let dh = (maxy - miny) - (initial_aabb.3 - initial_aabb.1);
            let d = if dw.abs() > dh.abs() { dw } else { dh };
            let (ax, min_ax, ay, min_ay) = handle.axes();
            if ax && ay {
                let w = (initial_aabb.2 - initial_aabb.0) + d;
                let h = (initial_aabb.3 - initial_aabb.1) + d;
                if min_ax {
                    minx = maxx - w;
                } else {
                    maxx = minx + w;
                }
                if min_ay {
                    miny = maxy - h;
                } else {
                    maxy = miny + h;
                }
            }
        }

        // Prevent inversion — enforce minimum size.
        let min_size = 1.0;
        if maxx - minx < min_size {
            maxx = minx + min_size;
        }
        if maxy - miny < min_size {
            maxy = miny + min_size;
        }

        let new_aabb = (minx, miny, maxx, maxy);

        for &(idx, ref geom, pos) in &self.drag_handle_initial_geoms {
            // The initial geometry is stored without the transform offset,
            // so shift the AABBs into geometry-local space.
            let (tx, ty) = pos;
            let local_initial = (
                initial_aabb.0 - tx,
                initial_aabb.1 - ty,
                initial_aabb.2 - tx,
                initial_aabb.3 - ty,
            );
            let local_new = (
                new_aabb.0 - tx,
                new_aabb.1 - ty,
                new_aabb.2 - tx,
                new_aabb.3 - ty,
            );
            self.document[idx].geometry = geom.remap_aabb(local_initial, local_new);
        }
    }

    pub fn end_handle_drag(&mut self) -> Vec<(usize, Geometry, Point, Geometry, Point)> {
        self.is_drawing = false;
        self.drag_start_world = None;
        self.drag_handle = None;
        self.drag_handle_initial_aabb = None;

        let mut result = Vec::new();
        for (idx, before_geom, before_pos) in self.drag_handle_initial_geoms.drain(..) {
            let after_geom = self.document[idx].geometry.clone();
            let after_pos = self.document[idx].transform.position;
            // Only record if something actually changed.
            if after_geom != before_geom {
                result.push((idx, before_geom, before_pos, after_geom, after_pos));
            }
        }
        result
    }
}

/// Axis-aligned bounding-box hit test for a primitive (in world space).
/// The transform.position offset is accounted for.
fn primitive_hit(prim: &Primitive, world: Point) -> bool {
    let (wx, wy) = world;
    let (tx, ty) = prim.transform.position;
    // Undo the transform so we test in geometry-local space.
    let (lx, ly) = (wx - tx, wy - ty);

    const PAD: f64 = 8.0; // hit-test padding in world units

    match &prim.geometry {
        Geometry::Stroke(pts) => {
            if pts.is_empty() {
                return false;
            }
            pts.windows(2)
                .any(|seg| point_to_segment_dist2(lx, ly, seg[0], seg[1]) < (PAD * PAD))
        }
        Geometry::Line { start, end } | Geometry::Arrow { start, end } => {
            point_to_segment_dist2(lx, ly, *start, *end) < (PAD * PAD)
        }
        Geometry::Rect {
            origin: (ox, oy),
            size: (w, h),
        } => lx >= ox - PAD && lx <= ox + w + PAD && ly >= oy - PAD && ly <= oy + h + PAD,
        Geometry::Oval {
            origin: (ox, oy),
            size: (w, h),
        } => {
            let (rx, ry) = (w / 2.0, h / 2.0);
            let (cx, cy) = (ox + rx, oy + ry);
            if rx <= 0.0 || ry <= 0.0 {
                return false;
            }
            let (nx, ny) = ((lx - cx) / (rx + PAD), (ly - cy) / (ry + PAD));
            nx * nx + ny * ny <= 1.0
        }
        _ => false,
    }
}

fn point_to_segment_dist2(px: f64, py: f64, (ax, ay): Point, (bx, by): Point) -> f64 {
    let (dx, dy) = (bx - ax, by - ay);
    let len2 = dx * dx + dy * dy;
    if len2 == 0.0 {
        let (ex, ey) = (px - ax, py - ay);
        return ex * ex + ey * ey;
    }
    let t = ((px - ax) * dx + (py - ay) * dy) / len2;
    let t = t.clamp(0.0, 1.0);
    let (qx, qy) = (ax + t * dx, ay + t * dy);
    let (ex, ey) = (px - qx, py - qy);
    ex * ex + ey * ey
}
