use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, js_sys};

use crate::canvas::{
    background::BackgroundKind,
    primitives::{
        Primitive, geometry::primitives_aabb, handle::HandleKind, renderer::PrimitiveRenderer,
    },
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

        if state.background_pattern != BackgroundKind::None {
            state.background_pattern.draw(&ctx, canvas, state);
        }

        if let (Some(start), Some(current)) =
            (state.selection_drag_start, state.selection_drag_current)
        {
            let (x0, y0) = start;
            let (x1, y1) = current;
            let minx = x0.min(x1);
            let miny = y0.min(y1);
            let w = (x1 - x0).abs();
            let h = (y1 - y0).abs();
            ctx.save();
            ctx.set_stroke_style_str("#1976d2");
            ctx.set_line_width(1.5);
            ctx.set_fill_style_str("rgba(25, 118, 210, 0.18)");
            ctx.fill_rect(minx, miny, w, h);
            ctx.stroke_rect(minx, miny, w, h);
            ctx.restore();
        }

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

        if let Some(active) = &state.active
            && let Some(prev) = active.preview(&state.current_style)
        {
            for p in &prev {
                PrimitiveRenderer::draw(&ctx, p);
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
    let bb_color = "#4A90E2";
    let handle_color = "#ffffff";

    let Some((minx, miny, maxx, maxy)) = primitives_aabb(prims.iter().copied()) else {
        return;
    };
    let pad = 6.0 / zoom;
    let handle_radius = 5.0 / zoom;

    ctx.save();
    ctx.set_stroke_style_str(bb_color);
    ctx.set_line_width(1.5 / zoom);
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

    // Draw handles.
    ctx.set_line_dash(&js_sys::Array::new()).unwrap();
    ctx.set_fill_style_str(handle_color);
    ctx.set_stroke_style_str(bb_color);
    ctx.set_line_width(1.5 / zoom);

    for &kind in HandleKind::ALL {
        let (hx, hy) = kind.position(minx, miny, maxx, maxy);
        ctx.begin_path();
        ctx.arc(hx, hy, handle_radius, 0.0, std::f64::consts::TAU)
            .unwrap();
        ctx.fill();
        ctx.stroke();
    }

    ctx.restore();
}
