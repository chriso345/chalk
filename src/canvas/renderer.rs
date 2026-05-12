use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::canvas::{primitives::renderer::PrimitiveRenderer, state::WhiteboardState};

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

        for primitive in &state.document {
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
