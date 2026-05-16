use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, js_sys};

use crate::canvas::{
    primitives::{Primitive, geometry::primitives_aabb, renderer::PrimitiveRenderer},
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

        // Collect all selected primitives and draw one combined bounding box
        let selected_prims: Vec<&Primitive> = state
            .document
            .iter()
            .enumerate()
            .filter(|(i, _)| state.selected.contains(i))
            .map(|(_, p)| p)
            .collect();

        if !selected_prims.is_empty() {
            draw_selection_highlight(&ctx, &selected_prims, state.vt.zoom);
        }

        for primitive in state.document.iter() {
            PrimitiveRenderer::draw(&ctx, primitive);
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

fn draw_selection_highlight(ctx: &CanvasRenderingContext2d, prims: &[&Primitive], zoom: f64) {
    let Some((minx, miny, maxx, maxy)) = primitives_aabb(prims.iter().copied()) else {
        return;
    };
    let pad = 6.0;
    let line_width = 2.5;
    ctx.save();
    ctx.set_stroke_style_str("#4A90E2");
    ctx.set_line_width(line_width / zoom);
    ctx.set_line_dash(&js_sys::Array::of2(
        &wasm_bindgen::JsValue::from_f64(6.0 / zoom),
        &wasm_bindgen::JsValue::from_f64(4.0 / zoom),
    ))
    .unwrap();
    ctx.stroke_rect(
        minx - pad,
        miny - pad,
        (maxx - minx) + pad * 2.0,
        (maxy - miny) + pad * 2.0,
    );
    ctx.restore();
}
