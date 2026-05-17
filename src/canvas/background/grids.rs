use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::canvas::state::WhiteboardState;

use super::*;

pub const LINE_WIDTH: f64 = 1.0;

pub fn draw(ctx: &CanvasRenderingContext2d, canvas: &HtmlCanvasElement, state: &WhiteboardState) {
    let w = canvas.width();
    let h = canvas.height();

    let zoom = state.vt.zoom;
    let offset_x = state.vt.offset_x;
    let offset_y = state.vt.offset_y;

    let spacing = get_screen_spacing(zoom);

    let phase_x = offset_x.rem_euclid(spacing);
    let phase_y = offset_y.rem_euclid(spacing);

    let dark_mode = state.style.is_dark();

    let key = BackgroundCacheKey {
        spacing_q: (spacing / CACHE_GRID).round() as u32,
        phase_x_q: (phase_x / CACHE_GRID).round() as u32,
        phase_y_q: (phase_y / CACHE_GRID).round() as u32,
        width: w,
        height: h,
        dark_mode,
        variant: 1,
    };

    let offscreen = with_cache(key, w, h, |ctx2| {
        ctx2.set_line_width(LINE_WIDTH);
        ctx2.set_global_alpha(1.0);

        let color = state.style.get_canvas_bg_color();
        ctx2.set_stroke_style_str(color);

        let cols = (w as f64 / spacing).ceil() as i32 + 1;
        let rows = (h as f64 / spacing).ceil() as i32 + 1;

        // vertical lines
        for c in 0..cols {
            let x = phase_x + c as f64 * spacing;
            ctx2.begin_path();
            ctx2.move_to(x, 0.0);
            ctx2.line_to(x, h as f64);
            ctx2.stroke();
        }

        // horizontal lines
        for r in 0..rows {
            let y = phase_y + r as f64 * spacing;
            ctx2.begin_path();
            ctx2.move_to(0.0, y);
            ctx2.line_to(w as f64, y);
            ctx2.stroke();
        }
    });

    ctx.draw_image_with_offscreen_canvas(&offscreen, 0.0, 0.0)
        .unwrap();
}
