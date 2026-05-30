use leptos::prelude::*;
use web_sys::{HtmlCanvasElement, PointerEvent, WheelEvent};

use crate::canvas::action::ChalkAction;
use crate::canvas::tool::Tool;
use crate::canvas::{renderer::WhiteboardRenderer, state::WhiteboardState};
use crate::signals::ChalkSignals;

pub const MIN_ZOOM: f64 = 0.10;
pub const MAX_ZOOM: f64 = 30.0;
pub const ZOOM_SENSITIVITY: f64 = 0.001;

/// Redraws the canvas from the current state. Call this after any mutation.
pub fn redraw(canvas: &HtmlCanvasElement, state: &WhiteboardState) {
    WhiteboardRenderer::draw(canvas, state);
}

/// Returns a closure that calls `redraw` if the canvas node is available.
///
/// Typical usage inside an `Effect`:
/// ```ignore
/// let repaint = make_repaint(canvas_ref, state);
/// Effect::new(move |_| { ..mutate state..; repaint(); });
/// ```
pub fn make_repaint(
    canvas_ref: NodeRef<leptos::html::Canvas>,
    state: RwSignal<WhiteboardState>,
) -> impl Fn() + Clone {
    move || {
        let Some(canvas) = canvas_ref.get() else {
            return;
        };
        state.with_untracked(|s| redraw(&canvas, s));
    }
}

pub struct WhiteboardController;

impl WhiteboardController {
    pub fn on_pointer_down(
        e: PointerEvent,
        canvas_ref: NodeRef<leptos::html::Canvas>,
        state: RwSignal<WhiteboardState>,
    ) {
        e.prevent_default();
        let Some(canvas) = canvas_ref.get() else {
            return;
        };
        let _ = canvas.set_pointer_capture(e.pointer_id());

        let screen = (e.client_x() as f64, e.client_y() as f64);
        let is_middle = e.button() == 1;
        let ctrl = e.ctrl_key();

        state.update(|s| {
            if !is_middle && s.tool == Tool::Pointer {
                let world = s.vt.screen_to_world(screen.0, screen.1);
                let zoom = s.vt.zoom;

                // Handles take priority over everything.
                if let Some(handle) = s.hit_test_handle(world, zoom) {
                    s.drag_handle = Some(handle);
                    s.begin_handle_drag(screen);
                    return;
                }

                let has_selection = s.apply_selection(world, ctrl);
                if has_selection {
                    s.begin_drag(screen);
                    return;
                }

                // Start selection drag if nothing is hit and not ctrl
                if !ctrl && s.hit_test(world).is_none() {
                    s.selection_drag_start = Some(screen);
                    s.selection_drag_current = Some(screen);
                    return;
                }
            }
            s.begin_drawing(screen, is_middle);
        });
    }

    pub fn on_pointer_move(
        e: PointerEvent,
        canvas_ref: NodeRef<leptos::html::Canvas>,
        state: RwSignal<WhiteboardState>,
    ) {
        if !state.with_untracked(|s| s.is_drawing || s.selection_drag_start.is_some()) {
            return;
        }
        e.prevent_default();

        let screen = (e.client_x() as f64, e.client_y() as f64);
        let is_middle = (e.buttons() & 4) != 0;

        state.update(|s| {
            if s.tool == Tool::Pointer && !is_middle {
                if s.selection_drag_start.is_some() {
                    s.selection_drag_current = Some(screen);
                    return;
                }
                if s.drag_handle.is_some() {
                    s.update_handle_drag(screen, e.shift_key());
                    return;
                }
                if !s.selected.is_empty() {
                    s.update_drag(screen);
                    return;
                }
            }
            s.update_drawing(screen, e.shift_key(), is_middle);
        });

        let Some(canvas) = canvas_ref.get() else {
            return;
        };
        state.with_untracked(|s| redraw(&canvas, s));
    }

    pub fn on_pointer_up(
        canvas_ref: NodeRef<leptos::html::Canvas>,
        state: RwSignal<WhiteboardState>,
        signals: ChalkSignals,
    ) {
        state.update(|s| {
            if s.tool == Tool::Pointer {
                // Commit selection drag
                if let (Some(start), Some(end)) = (
                    s.selection_drag_start.take(),
                    s.selection_drag_current.take(),
                ) {
                    let (x0, y0) = start;
                    let (x1, y1) = end;
                    let minx = x0.min(x1);
                    let miny = y0.min(y1);
                    let maxx = x0.max(x1);
                    let maxy = y0.max(y1);
                    let vt = s.vt;
                    let world0 = vt.screen_to_world(minx, miny);
                    let world1 = vt.screen_to_world(maxx, maxy);
                    s.selected.clear();
                    for (i, prim) in s.document.iter().enumerate() {
                        let (minx, miny, maxx, maxy) =
                            crate::canvas::primitives::geometry::expand_aabb(
                                (
                                    f64::INFINITY,
                                    f64::INFINITY,
                                    f64::NEG_INFINITY,
                                    f64::NEG_INFINITY,
                                ),
                                prim,
                            );
                        if minx >= world0.0
                            && maxx <= world1.0
                            && miny >= world0.1
                            && maxy <= world1.1
                        {
                            s.selected.insert(i);
                        }
                    }
                    return;
                }
                // Handle resize commit.
                if s.drag_handle.is_some() {
                    let moves = s.end_handle_drag();
                    if !moves.is_empty() {
                        let actions = moves
                            .into_iter()
                            .map(|(idx, before_geom, before_pos, after_geom, after_pos)| {
                                let mut before = s.document[idx].clone();
                                before.geometry = before_geom;
                                before.transform.position = before_pos;
                                let mut after = s.document[idx].clone();
                                after.geometry = after_geom;
                                after.transform.position = after_pos;
                                ChalkAction::Transform {
                                    before,
                                    after,
                                    index: idx,
                                }
                            })
                            .collect::<Vec<_>>();
                        let action = if actions.len() == 1 {
                            actions.into_iter().next().unwrap()
                        } else {
                            ChalkAction::Batch { actions }
                        };
                        s.history.apply(&mut s.document, action);
                    }
                    return;
                }

                // Normal drag commit.
                if !s.selected.is_empty() {
                    let moves = s.end_drag();
                    if !moves.is_empty() {
                        let actions = moves
                            .into_iter()
                            .map(|(idx, before_pos, _)| {
                                let mut before = s.document[idx].clone();
                                before.transform.position = before_pos;
                                let after = s.document[idx].clone();
                                ChalkAction::Transform {
                                    before,
                                    after,
                                    index: idx,
                                }
                            })
                            .collect::<Vec<_>>();
                        let action = if actions.len() == 1 {
                            actions.into_iter().next().unwrap()
                        } else {
                            ChalkAction::Batch { actions }
                        };
                        s.history.apply(&mut s.document, action);
                    }
                    return;
                }
            }

            if let Some(primitive) = s.end_drawing() {
                s.history
                    .apply(&mut s.document, ChalkAction::Add { primitive });
                if !signals.lock_tool.get() {
                    signals.tool.set(Tool::Pointer);
                }
            }
        });

        let Some(canvas) = canvas_ref.get() else {
            return;
        };
        state.with_untracked(|s| redraw(&canvas, s));
    }

    pub fn on_wheel(
        e: WheelEvent,
        canvas_ref: NodeRef<leptos::html::Canvas>,
        state: RwSignal<WhiteboardState>,
        signals: ChalkSignals,
    ) {
        e.prevent_default();
        let factor = 1.0 - e.delta_y() * ZOOM_SENSITIVITY;

        state.update(|s| {
            s.vt = s.vt.zoom_towards(
                e.client_x() as f64,
                e.client_y() as f64,
                factor,
                MIN_ZOOM,
                MAX_ZOOM,
            );
        });

        let new_zoom = state.with_untracked(|s| s.vt.zoom);
        signals.zoom.set((new_zoom * 100.0).round() as u32);

        let Some(canvas) = canvas_ref.get() else {
            return;
        };
        state.with_untracked(|s| redraw(&canvas, s));
    }
}
