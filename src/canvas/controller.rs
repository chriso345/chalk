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

        state.update(|s| {
            if !is_middle && s.tool == Tool::Pointer {
                // Hit-test in world space.
                let world = s.vt.screen_to_world(screen.0, screen.1);
                s.selected = s.hit_test(world);
                if s.selected.is_some() {
                    s.begin_drag(screen);
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
        if !state.with_untracked(|s| s.is_drawing) {
            return;
        }
        e.prevent_default();

        let screen = (e.client_x() as f64, e.client_y() as f64);
        let is_middle = (e.buttons() & 4) != 0;

        state.update(|s| {
            // If dragging a selected primitive, use update_drag instead.
            if s.tool == Tool::Pointer && s.selected.is_some() && !is_middle {
                s.update_drag(screen);
            } else {
                s.update_drawing(screen, e.shift_key(), is_middle);
            }
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
            if s.tool == Tool::Pointer && s.selected.is_some() {
                // Commit drag to history if the primitive moved.
                if let Some((idx, before_pos, _)) = s.end_drag() {
                    // Reconstruct before/after Primitives for the history entry.
                    let mut before = s.document[idx].clone();
                    before.transform.position = before_pos;
                    let after = s.document[idx].clone(); // already has after_pos applied
                    s.history.apply(
                        &mut s.document,
                        ChalkAction::Transform {
                            before,
                            after,
                            index: idx,
                        },
                    );
                }
                return;
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
