use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::canvas::state::WhiteboardState;

use super::*;

pub const DOT_RADIUS: f64 = 1.0;

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
        variant: 0,
    };

    let offscreen = with_cache(key, w, h, |ctx2| {
        ctx2.set_global_alpha(1.0);
        ctx2.set_fill_style_str(state.style.get_canvas_bg_color());

        let cols = (w as f64 / spacing).ceil() as i32 + 1;
        let rows = (h as f64 / spacing).ceil() as i32 + 1;

        for c in 0..cols {
            for r in 0..rows {
                let x = phase_x + c as f64 * spacing;
                let y = phase_y + r as f64 * spacing;

                ctx2.begin_path();
                ctx2.arc(x, y, DOT_RADIUS, 0.0, std::f64::consts::TAU)
                    .unwrap();
                ctx2.fill();
            }
        }
    });

    ctx.draw_image_with_offscreen_canvas(&offscreen, 0.0, 0.0)
        .unwrap();
}
