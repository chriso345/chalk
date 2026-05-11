use std::f64::consts::TAU;

use web_sys::CanvasRenderingContext2d;

use crate::canvas::primitives::{Primitive, shape::Shape};

pub struct PrimitiveRenderer;

impl PrimitiveRenderer {
    /// Draw any primitive. Groups recurse naturally.
    pub fn draw(
        ctx: &CanvasRenderingContext2d,
        primitive: &Primitive,
        stroke_width: f64,
        zoom: f64,
    ) {
        match primitive {
            Primitive::Stroke(pts) => Self::draw_stroke(ctx, pts, stroke_width, zoom),
            Primitive::Shape(shape) => Self::draw_shape(ctx, shape),
            Primitive::Group(children) => {
                for child in children {
                    Self::draw(ctx, child, stroke_width, zoom);
                }
            }
        }
    }

    fn draw_stroke(
        ctx: &CanvasRenderingContext2d,
        pts: &[(f64, f64)],
        stroke_width: f64,
        zoom: f64,
    ) {
        match pts {
            [] => {}
            [(x, y)] => {
                ctx.begin_path();
                ctx.arc(*x, *y, stroke_width / zoom / 2.0, 0.0, TAU)
                    .unwrap();
                ctx.fill();
            }
            // Use Catmull-Rom smoothing for stroke sampling
            [(x0, y0), rest @ ..] => {
                ctx.begin_path();

                ctx.set_line_join("round");
                ctx.set_line_cap("round");
                ctx.set_line_width(stroke_width / zoom);

                let mut points: Vec<(f64, f64)> = Vec::with_capacity(pts.len() + 2);

                points.push((*x0, *y0));
                points.extend_from_slice(rest);

                if let Some(last) = points.last().copied() {
                    points.push(last);
                }

                if points.len() < 4 {
                    ctx.move_to(points[0].0, points[0].1);
                    for &(x, y) in &points[1..] {
                        ctx.line_to(x, y);
                    }
                    ctx.stroke();
                    return;
                }

                ctx.move_to(points[0].0, points[0].1);

                for i in 0..points.len() - 3 {
                    let p0 = points[i];
                    let p1 = points[i + 1];
                    let p2 = points[i + 2];
                    let p3 = points[i + 3];

                    let cp1 = (p1.0 + (p2.0 - p0.0) / 6.0, p1.1 + (p2.1 - p0.1) / 6.0);

                    let cp2 = (p2.0 - (p3.0 - p1.0) / 6.0, p2.1 - (p3.1 - p1.1) / 6.0);

                    ctx.bezier_curve_to(cp1.0, cp1.1, cp2.0, cp2.1, p2.0, p2.1);
                }

                ctx.stroke();
            }
        }
    }

    fn draw_shape(ctx: &CanvasRenderingContext2d, shape: &Shape) {
        match shape {
            Shape::Line {
                start: (x0, y0),
                end: (x1, y1),
            } => {
                ctx.begin_path();
                ctx.move_to(*x0, *y0);
                ctx.line_to(*x1, *y1);
                ctx.stroke();
            }
            Shape::Arrow {
                start: (x0, y0),
                end: (x1, y1),
            } => {
                let dx = x1 - x0;
                let dy = y1 - y0;
                let len = (dx * dx + dy * dy).sqrt();
                if len == 0.0 {
                    return;
                }
                let ux = dx / len;
                let uy = dy / len;

                // Arrowhead size scales with stroke width but has a minimum size.
                let head_len = (ctx.line_width() * 4.0).max(10.0);
                let head_width = head_len / 2.0;

                ctx.begin_path();
                ctx.move_to(*x0, *y0);
                ctx.line_to(*x1, *y1);
                ctx.stroke();

                // Draw arrowhead as a filled triangle.
                ctx.begin_path();
                ctx.move_to(*x1, *y1);
                ctx.line_to(
                    x1 - ux * head_len + uy * head_width,
                    y1 - uy * head_len - ux * head_width,
                );
                ctx.line_to(
                    x1 - ux * head_len - uy * head_width,
                    y1 - uy * head_len + ux * head_width,
                );
                ctx.close_path();
                ctx.fill();
            }
            Shape::Rect {
                origin: (x, y),
                size: (w, h),
            } => {
                ctx.begin_path();
                ctx.rect(*x, *y, *w, *h);
                ctx.stroke();
            }
            Shape::Oval {
                origin: (x, y),
                size: (w, h),
            } => {
                let rx = *w / 2.0;
                let ry = *h / 2.0;

                let cx = *x + rx;
                let cy = *y + ry;

                ctx.begin_path();

                ctx.ellipse(
                    cx, cy, rx, ry, 0.0, // rotation
                    0.0, // start angle
                    TAU, // end angle
                )
                .unwrap();

                ctx.stroke();
            }
        }
    }
}
