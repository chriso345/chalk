use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, PointerEvent, WheelEvent};

use crate::canvas::types::{Stroke, ViewTransform};

pub const BG_COLOR: &str = "#F2F0EF";
const STROKE_COLOR: &str = "#1a1a18";
const STROKE_WIDTH: f64 = 2.0;
const MIN_ZOOM: f64 = 0.10;
const MAX_ZOOM: f64 = 30.0;
const ZOOM_SENSITIVITY: f64 = 0.001;

fn get_ctx(canvas: &HtmlCanvasElement) -> Option<CanvasRenderingContext2d> {
    let obj = canvas.get_context("2d").ok()??;
    obj.dyn_into::<CanvasRenderingContext2d>().ok()
}

fn redraw(canvas: &HtmlCanvasElement, strokes: &[Stroke], vt: ViewTransform) {
    let Some(ctx) = get_ctx(canvas) else { return };
    let w = canvas.width() as f64;
    let h = canvas.height() as f64;

    ctx.set_fill_style_str(BG_COLOR);
    ctx.fill_rect(0.0, 0.0, w, h);

    ctx.save();
    ctx.translate(vt.offset_x, vt.offset_y).unwrap();
    ctx.scale(vt.zoom, vt.zoom).unwrap();

    ctx.set_stroke_style_str(STROKE_COLOR);
    ctx.set_line_width(STROKE_WIDTH / vt.zoom);
    ctx.set_line_cap("round");
    ctx.set_line_join("round");

    for stroke in strokes {
        match stroke.len() {
            0 => {}
            1 => {
                let (x, y) = stroke[0];
                ctx.begin_path();
                ctx.arc(
                    x,
                    y,
                    STROKE_WIDTH / vt.zoom / 2.0,
                    0.0,
                    std::f64::consts::TAU,
                )
                .unwrap();
                ctx.set_fill_style_str(STROKE_COLOR);
                ctx.fill();
            }
            _ => {
                ctx.begin_path();
                let (sx, sy) = stroke[0];
                ctx.move_to(sx, sy);
                for &(x, y) in &stroke[1..] {
                    ctx.line_to(x, y);
                }
                ctx.stroke();
            }
        }
    }

    ctx.restore();
}

/// Exposes the current zoom level so the overlay can display it.
#[component]
pub fn Whiteboard(
    /// Write-only signal the canvas pushes zoom updates into.
    set_zoom_pct: WriteSignal<u32>,

    should_clear: ReadSignal<u32>,
) -> impl IntoView {
    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();

    let strokes = RwSignal::<Vec<Stroke>>::new(vec![]);
    let is_drawing = RwSignal::new(false);
    let vt = RwSignal::new(ViewTransform::default());

    // Clear canvas trigger
    {
        let canvas_ref = canvas_ref.clone();
        Effect::new(move |_| {
            let _ = should_clear.get();
            strokes.set(vec![]);
            let Some(canvas) = canvas_ref.get() else {
                return;
            };
            let Some(ctx) = get_ctx(&canvas) else { return };
            ctx.set_fill_style_str(BG_COLOR);
            ctx.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        });
    }

    // Resize canvas trigger
    {
        let canvas_ref = canvas_ref.clone();
        Effect::new(move |_| {
            let Some(canvas) = canvas_ref.get() else {
                return;
            };
            let win = web_sys::window().unwrap();
            let w = win.inner_width().unwrap().as_f64().unwrap() as u32;
            let h = win.inner_height().unwrap().as_f64().unwrap() as u32;
            canvas.set_width(w);
            canvas.set_height(h);

            if let Some(ctx) = get_ctx(&canvas) {
                ctx.set_fill_style_str(BG_COLOR);
                ctx.fill_rect(0.0, 0.0, w as f64, h as f64);
            }

            let canvas_c = canvas.clone();
            let closure = Closure::<dyn Fn()>::new(move || {
                let win = web_sys::window().unwrap();
                let nw = win.inner_width().unwrap().as_f64().unwrap() as u32;
                let nh = win.inner_height().unwrap().as_f64().unwrap() as u32;
                canvas_c.set_width(nw);
                canvas_c.set_height(nh);
                strokes.with_untracked(|s| redraw(&canvas_c, s, vt.get_untracked()));
            });
            win.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        });
    }

    let on_pointer_down = {
        let canvas_ref = canvas_ref.clone();
        Callback::new(move |e: PointerEvent| {
            e.prevent_default();
            let Some(canvas) = canvas_ref.get() else {
                return;
            };
            let _ = canvas.set_pointer_capture(e.pointer_id());
            let transform = vt.get_untracked();
            let (wx, wy) = transform.screen_to_world(e.client_x() as f64, e.client_y() as f64);
            strokes.update(|s| s.push(vec![(wx, wy)]));
            is_drawing.set(true);
        })
    };

    let on_pointer_move = {
        let canvas_ref = canvas_ref.clone();
        Callback::new(move |e: PointerEvent| {
            if !is_drawing.get_untracked() {
                return;
            }
            e.prevent_default();
            let transform = vt.get_untracked();
            let (wx, wy) = transform.screen_to_world(e.client_x() as f64, e.client_y() as f64);
            strokes.update(|s| {
                if let Some(last) = s.last_mut() {
                    last.push((wx, wy));
                }
            });

            // Incremental draw - only the last segment
            let Some(canvas) = canvas_ref.get() else {
                return;
            };
            let Some(ctx) = get_ctx(&canvas) else { return };
            strokes.with_untracked(|s| {
                let Some(stroke) = s.last() else { return };
                let n = stroke.len();
                if n < 2 {
                    return;
                }
                let (x0, y0) = stroke[n - 2];
                let (x1, y1) = stroke[n - 1];
                ctx.save();
                ctx.translate(transform.offset_x, transform.offset_y)
                    .unwrap();
                ctx.scale(transform.zoom, transform.zoom).unwrap();
                ctx.set_stroke_style_str(STROKE_COLOR);
                ctx.set_line_width(STROKE_WIDTH / transform.zoom);
                ctx.set_line_cap("round");
                ctx.set_line_join("round");
                ctx.begin_path();
                ctx.move_to(x0, y0);
                ctx.line_to(x1, y1);
                ctx.stroke();
                ctx.restore();
            });
        })
    };

    let on_pointer_up = Callback::new(move |_: PointerEvent| {
        is_drawing.set(false);
    });

    let on_wheel = {
        let canvas_ref = canvas_ref.clone();
        Callback::new(move |e: WheelEvent| {
            e.prevent_default();
            let factor = 1.0 - e.delta_y() * ZOOM_SENSITIVITY;
            let new_vt = vt.get_untracked().zoom_towards(
                e.client_x() as f64,
                e.client_y() as f64,
                factor,
                MIN_ZOOM,
                MAX_ZOOM,
            );
            vt.set(new_vt);
            set_zoom_pct.set((new_vt.zoom * 100.0).round() as u32);

            let Some(canvas) = canvas_ref.get() else {
                return;
            };
            strokes.with_untracked(|s| redraw(&canvas, s, new_vt));
        })
    };

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
