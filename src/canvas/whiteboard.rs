use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{PointerEvent, WheelEvent};

use crate::canvas::{
    controller::{MAX_ZOOM, MIN_ZOOM, WhiteboardController, make_repaint},
    state::WhiteboardState,
};
use crate::signals::ChalkSignals;

#[component]
pub fn Whiteboard(signals: ChalkSignals) -> impl IntoView {
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();
    let state = RwSignal::new(WhiteboardState::new());
    let repaint = make_repaint(canvas_ref, state);

    Effect::new({
        let repaint = repaint.clone();
        move |_| {
            let _ = signals.clear.get();
            state.update(|s| s.clear());
            repaint();
        }
    });

    Effect::new({
        let repaint = repaint.clone();
        move |_| {
            let _ = signals.undo.get();
            state.update(|s| s.undo());
            repaint();
        }
    });

    Effect::new({
        let repaint = repaint.clone();
        move |_| {
            let _ = signals.redo.get();
            state.update(|s| s.redo());
            repaint();
        }
    });

    Effect::new({
        let repaint = repaint.clone();
        let canvas_ref = canvas_ref.clone();
        move |_| {
            let zoom_percent = signals.zoom.get();
            let target_zoom = (zoom_percent as f64 / 100.0).clamp(MIN_ZOOM, MAX_ZOOM);

            let Some(canvas) = canvas_ref.get() else {
                return;
            };
            let center_x = canvas.width() as f64 / 2.0;
            let center_y = canvas.height() as f64 / 2.0;

            state.update(|s| {
                s.set_zoom_centered(target_zoom, center_x, center_y, MIN_ZOOM, MAX_ZOOM);
            });
            repaint();
        }
    });

    Effect::new({
        let repaint = repaint.clone();
        move |_| {
            let _ = signals.dark_mode.get();
            state.update(|s| s.change_theme(signals.dark_mode.get()));
            repaint();
        }
    });

    Effect::new({
        let repaint = repaint.clone();
        let canvas_ref = canvas_ref.clone();
        move |_| {
            let Some(canvas) = canvas_ref.get() else {
                return;
            };
            let win = web_sys::window().unwrap();

            let set_size = |c: &web_sys::HtmlCanvasElement| {
                let win = web_sys::window().unwrap();
                c.set_width(win.inner_width().unwrap().as_f64().unwrap() as u32);
                c.set_height(win.inner_height().unwrap().as_f64().unwrap() as u32);
            };

            set_size(&canvas);
            repaint();

            let repaint_resize = repaint.clone();
            let canvas_c = canvas.clone();
            let closure = Closure::<dyn Fn()>::new(move || {
                set_size(&canvas_c);
                repaint_resize();
            });
            win.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }
    });

    let on_pointer_down = Callback::new(move |e: PointerEvent| {
        WhiteboardController::on_pointer_down(e, canvas_ref, state);
    });

    let on_pointer_move = Callback::new(move |e: PointerEvent| {
        WhiteboardController::on_pointer_move(e, canvas_ref, state);
    });

    let on_pointer_up = Callback::new(move |_: PointerEvent| {
        WhiteboardController::on_pointer_up(canvas_ref, state);
    });

    let on_wheel = Callback::new(move |e: WheelEvent| {
        WhiteboardController::on_wheel(e, canvas_ref, state, signals);
    });

    view! {
        <canvas
            node_ref=canvas_ref
            style="display:block;position:fixed;inset:0;touch-action:none;cursor:crosshair;"
            on:pointerdown=move |e| on_pointer_down.run(e)
            on:pointermove=move |e| on_pointer_move.run(e)
            on:pointerup=move |e| on_pointer_up.run(e)
            on:pointercancel=move |e| on_pointer_up.run(e)
            on:wheel=move |e| on_wheel.run(e)
        />
    }
}
