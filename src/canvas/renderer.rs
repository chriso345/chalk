use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, js_sys};

use crate::canvas::{
    primitives::{Primitive, renderer::PrimitiveRenderer},
    state::WhiteboardState,
};

pub struct WhiteboardRenderer;

impl WhiteboardRenderer {
    pub fn draw(canvas: &HtmlCanvasElement, state: &WhiteboardState) {
        let Some(ctx) = get_ctx(canvas) else { return };

        let w = canvas.width() as f64;
        let h = canvas.height() as f64;

        ctx.set_fill_style_str(state.style.get_bg());
        ctx.fill_rect(0.0, 0.0, w, h);

        ctx.save();
        ctx.translate(state.vt.offset_x, state.vt.offset_y).unwrap();
        ctx.scale(state.vt.zoom, state.vt.zoom).unwrap();

        for (i, primitive) in state.document.iter().enumerate() {
            PrimitiveRenderer::draw(&ctx, primitive);

            // Draw selection highlight
            if state.selected == Some(i) {
                draw_selection_highlight(&ctx, primitive);
            }
        }

        if let Some(active) = &state.active {
            if let Some(prev) = active.preview(&state.current_style) {
                for p in &prev {
                    PrimitiveRenderer::draw(&ctx, p);
                }
            }
        }

        ctx.restore();
    }
}

pub fn get_ctx(canvas: &HtmlCanvasElement) -> Option<CanvasRenderingContext2d> {
    let obj = canvas.get_context("2d").ok()??;
    obj.dyn_into::<CanvasRenderingContext2d>().ok()
}

fn draw_selection_highlight(ctx: &CanvasRenderingContext2d, prim: &Primitive) {
    use crate::canvas::primitives::Geometry;
    let (tx, ty) = prim.transform.position;
    const PAD: f64 = 6.0;

    ctx.save();
    ctx.set_stroke_style_str("#4A90E2");
    ctx.set_line_width(1.5);
    ctx.set_line_dash(&js_sys::Array::of2(
        &wasm_bindgen::JsValue::from_f64(6.0),
        &wasm_bindgen::JsValue::from_f64(4.0),
    ))
    .unwrap();

    match &prim.geometry {
        Geometry::Rect {
            origin: (ox, oy),
            size: (w, h),
        } => {
            ctx.stroke_rect(tx + ox - PAD, ty + oy - PAD, w + PAD * 2.0, h + PAD * 2.0);
        }
        Geometry::Oval {
            origin: (ox, oy),
            size: (w, h),
        } => {
            ctx.stroke_rect(tx + ox - PAD, ty + oy - PAD, w + PAD * 2.0, h + PAD * 2.0);
        }
        Geometry::Stroke(pts) | Geometry::Polygon { points: pts } => {
            if let Some(bb) = bounding_box(pts) {
                ctx.stroke_rect(
                    tx + bb.0 - PAD,
                    ty + bb.1 - PAD,
                    bb.2 + PAD * 2.0,
                    bb.3 + PAD * 2.0,
                );
            }
        }
        Geometry::Line { start, end } | Geometry::Arrow { start, end } => {
            let (x0, y0) = start;
            let (x1, y1) = end;
            let (minx, miny) = (x0.min(*x1), y0.min(*y1));
            let (w, h) = ((x0 - x1).abs(), (y0 - y1).abs());
            ctx.stroke_rect(
                tx + minx - PAD,
                ty + miny - PAD,
                w + PAD * 2.0,
                h + PAD * 2.0,
            );
        }
        _ => {}
    }
    ctx.restore();
}

fn bounding_box(pts: &[(f64, f64)]) -> Option<(f64, f64, f64, f64)> {
    if pts.is_empty() {
        return None;
    }
    let (mut minx, mut miny) = pts[0];
    let (mut maxx, mut maxy) = pts[0];
    for &(x, y) in pts {
        minx = minx.min(x);
        miny = miny.min(y);
        maxx = maxx.max(x);
        maxy = maxy.max(y);
    }
    Some((minx, miny, maxx - minx, maxy - miny))
}
