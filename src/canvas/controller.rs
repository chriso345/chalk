use leptos::prelude::*;
use web_sys::{HtmlCanvasElement, PointerEvent, WheelEvent};

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

        state.update(|s| s.begin_drawing((e.client_x() as f64, e.client_y() as f64)));
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

        state.update(|s| {
            s.update_drawing((e.client_x() as f64, e.client_y() as f64), e.shift_key())
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
            if let Some(primitive) = s.end_drawing() {
                s.history.push(primitive);
                signals.tool.set(Tool::Pointer);
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
