use std::f64::consts::TAU;
use web_sys::CanvasRenderingContext2d;

use crate::canvas::primitives::{Primitive, geometry::Geometry};

pub struct PrimitiveRenderer;

impl PrimitiveRenderer {
    pub fn draw(ctx: &CanvasRenderingContext2d, primitive: &Primitive) {
        let style = &primitive.style;
        ctx.set_line_width(style.stroke_width);
        ctx.set_line_cap("round");
        ctx.set_line_join("round");

        if let Some(sc) = &style.stroke_color {
            ctx.set_stroke_style_str(sc);
        }
        if let Some(fc) = &style.fill_color {
            ctx.set_fill_style_str(fc);
        }

        let (tx, ty) = primitive.transform.position;
        if tx != 0.0 || ty != 0.0 {
            ctx.save();
            ctx.translate(tx, ty).unwrap();
            Self::draw_geometry(
                ctx,
                &primitive.geometry,
                style.stroke_width,
                style.stroke_color.as_deref().unwrap_or("#000000"),
            );
            ctx.restore();
        } else {
            Self::draw_geometry(
                ctx,
                &primitive.geometry,
                style.stroke_width,
                style.stroke_color.as_deref().unwrap_or("#000000"),
            );
        }
    }

    fn draw_geometry(
        ctx: &CanvasRenderingContext2d,
        geometry: &Geometry,
        stroke_width: f64,
        stroke_color: &str,
    ) {
        match geometry {
            Geometry::Stroke(pts) => Self::draw_stroke(ctx, pts, stroke_width, stroke_color),
            Geometry::Line { start, end } => Self::draw_line(ctx, *start, *end),
            Geometry::Arrow { start, end } => {
                Self::draw_arrow(ctx, *start, *end, stroke_width, stroke_color)
            }
            Geometry::Rect { origin, size } => Self::draw_rect(ctx, *origin, *size),
            Geometry::Oval { origin, size } => Self::draw_oval(ctx, *origin, *size),
            Geometry::Polygon { points } => Self::draw_polygon(ctx, points),
        }
    }

    fn draw_stroke(
        ctx: &CanvasRenderingContext2d,
        pts: &[(f64, f64)],
        screen_stroke: f64,
        stroke_color: &str,
    ) {
        match pts {
            [] => {}
            [(x, y)] => {
                ctx.begin_path();
                ctx.arc(*x, *y, screen_stroke / 2.0, 0.0, TAU).unwrap();
                ctx.set_fill_style_str(stroke_color);
                ctx.fill();
            }
            [(x0, y0), rest @ ..] => {
                let mut points = Vec::with_capacity(pts.len() + 2);
                points.push((*x0, *y0));
                points.extend_from_slice(rest);
                if let Some(last) = points.last().copied() {
                    points.push(last);
                }

                ctx.begin_path();
                ctx.move_to(points[0].0, points[0].1);

                if points.len() < 4 {
                    for &(x, y) in &points[1..] {
                        ctx.line_to(x, y);
                    }
                } else {
                    for i in 0..points.len() - 3 {
                        let (p0, p1, p2, p3) =
                            (points[i], points[i + 1], points[i + 2], points[i + 3]);
                        let cp1 = (p1.0 + (p2.0 - p0.0) / 6.0, p1.1 + (p2.1 - p0.1) / 6.0);
                        let cp2 = (p2.0 - (p3.0 - p1.0) / 6.0, p2.1 - (p3.1 - p1.1) / 6.0);
                        ctx.bezier_curve_to(cp1.0, cp1.1, cp2.0, cp2.1, p2.0, p2.1);
                    }
                }
                ctx.stroke();
            }
        }
    }

    fn draw_line(ctx: &CanvasRenderingContext2d, (x0, y0): (f64, f64), (x1, y1): (f64, f64)) {
        ctx.begin_path();
        ctx.move_to(x0, y0);
        ctx.line_to(x1, y1);
        ctx.stroke();
    }

    fn draw_arrow(
        ctx: &CanvasRenderingContext2d,
        (x0, y0): (f64, f64),
        (x1, y1): (f64, f64),
        stroke_width: f64,
        stroke_color: &str,
    ) {
        let dx = x1 - x0;
        let dy = y1 - y0;
        let len = (dx * dx + dy * dy).sqrt();
        if len == 0.0 {
            return;
        }
        let (ux, uy) = (dx / len, dy / len);
        let head_len = stroke_width * 4.0;
        let head_width = head_len / 2.0;

        // Stop the line at the base of the arrowhead, not the tip.
        let base_x = x1 - ux * head_len;
        let base_y = y1 - uy * head_len;

        ctx.begin_path();
        ctx.move_to(x0, y0);
        ctx.line_to(base_x, base_y);
        ctx.stroke();

        ctx.set_fill_style_str(stroke_color);

        ctx.begin_path();
        ctx.move_to(x1, y1); // tip
        ctx.line_to(base_x + uy * head_width, base_y - ux * head_width);
        ctx.line_to(base_x - uy * head_width, base_y + ux * head_width);
        ctx.close_path();
        ctx.fill();
    }

    fn draw_rect(ctx: &CanvasRenderingContext2d, (x, y): (f64, f64), (w, h): (f64, f64)) {
        ctx.begin_path();
        ctx.rect(x, y, w, h);
        ctx.stroke();
    }

    fn draw_oval(ctx: &CanvasRenderingContext2d, (x, y): (f64, f64), (w, h): (f64, f64)) {
        let (rx, ry) = (w / 2.0, h / 2.0);
        ctx.begin_path();
        ctx.ellipse(x + rx, y + ry, rx, ry, 0.0, 0.0, TAU).unwrap();
        ctx.stroke();
    }

    fn draw_polygon(ctx: &CanvasRenderingContext2d, points: &[(f64, f64)]) {
        // Placeholder - Polygon is not yet implemented.
        let _ = (ctx, points);
    }
}
