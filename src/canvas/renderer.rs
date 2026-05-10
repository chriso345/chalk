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

        ctx.set_stroke_style_str(state.style.get_stroke());
        ctx.set_fill_style_str(state.style.get_stroke());
        ctx.set_line_width(state.stroke_width / state.vt.zoom);
        ctx.set_line_cap("round");
        ctx.set_line_join("round");

        for primitive in state.history.visible() {
            PrimitiveRenderer::draw(&ctx, primitive, state.stroke_width, state.vt.zoom);
        }

        if let Some(active) = &state.active {
            if let Some(prev) = &active.preview() {
                for p in prev {
                    PrimitiveRenderer::draw(&ctx, p, state.stroke_width, state.vt.zoom);
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
