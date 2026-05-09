use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::canvas::{state::WhiteboardState, types::Stroke};

pub struct WhiteboardRenderer;

impl WhiteboardRenderer {
    pub fn draw(canvas: &HtmlCanvasElement, state: &WhiteboardState) {
        let Some(ctx) = get_ctx(canvas) else { return };

        let w = canvas.width() as f64;
        let h = canvas.height() as f64;

        ctx.set_fill_style_str(state.bg_color);
        ctx.fill_rect(0.0, 0.0, w, h);

        ctx.save();
        ctx.translate(state.vt.offset_x, state.vt.offset_y).unwrap();
        ctx.scale(state.vt.zoom, state.vt.zoom).unwrap();

        ctx.set_stroke_style_str(state.stroke_color);
        ctx.set_fill_style_str(state.stroke_color);
        ctx.set_line_width(state.stroke_width / state.vt.zoom);
        ctx.set_line_cap("round");
        ctx.set_line_join("round");

        for stroke in state.history.visible() {
            Self::draw_stroke(&ctx, stroke, state.stroke_width, state.vt.zoom);
        }

        if let Some(stroke) = &state.active_stroke {
            Self::draw_stroke(&ctx, stroke, state.stroke_width, state.vt.zoom);
        }

        ctx.restore();
    }

    fn draw_stroke(ctx: &CanvasRenderingContext2d, stroke: &Stroke, width: f64, zoom: f64) {
        match stroke.as_slice() {
            [] => {}
            [(x, y)] => {
                ctx.begin_path();
                ctx.arc(*x, *y, width / zoom / 2.0, 0.0, std::f64::consts::TAU)
                    .unwrap();
                ctx.fill();
            }
            [(x0, y0), rest @ ..] => {
                ctx.begin_path();
                ctx.move_to(*x0, *y0);
                for (x, y) in rest {
                    ctx.line_to(*x, *y);
                }
                ctx.stroke();
            }
        }
    }
}

pub fn get_ctx(canvas: &HtmlCanvasElement) -> Option<CanvasRenderingContext2d> {
    let obj = canvas.get_context("2d").ok()??;
    obj.dyn_into::<CanvasRenderingContext2d>().ok()
}
