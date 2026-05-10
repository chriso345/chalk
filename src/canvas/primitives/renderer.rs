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
