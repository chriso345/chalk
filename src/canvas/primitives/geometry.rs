use crate::canvas::types::Point;

/// The pure geometric description of a primitive — no style, no transform.
/// Polygon is reserved for future use.
#[derive(Clone, Debug, PartialEq)]
pub enum Geometry {
    /// Freehand polyline (smoothed at render time with Catmull-Rom).
    Stroke(Vec<Point>),
    Line {
        start: Point,
        end: Point,
    },
    Arrow {
        start: Point,
        end: Point,
    },
    Rect {
        origin: Point,
        size: (f64, f64),
    },
    Oval {
        origin: Point,
        size: (f64, f64),
    },
    // TODO: Add polygon support
    #[allow(dead_code)]
    Polygon {
        points: Vec<Point>,
    },
    // TODO: Add image support
    #[allow(dead_code)]
    Image {
        origin: Point,
        size: (f64, f64),
        data_url: String,
    },
    // TODO: Add text support
    #[allow(dead_code)]
    Text {
        origin: Point,
        content: String,
    },
}

impl Geometry {
    pub fn is_empty(&self) -> bool {
        match self {
            Geometry::Stroke(pts) => pts.is_empty(),
            _ => false,
        }
    }

    pub fn remap_aabb(
        &self,
        (old_minx, old_miny, old_maxx, old_maxy): (f64, f64, f64, f64),
        (new_minx, new_miny, new_maxx, new_maxy): (f64, f64, f64, f64),
    ) -> Self {
        let remap = |v: f64, old_min: f64, old_max: f64, new_min: f64, new_max: f64| -> f64 {
            let old_size = old_max - old_min;
            if old_size.abs() < f64::EPSILON {
                return new_min;
            }
            new_min + (v - old_min) / old_size * (new_max - new_min)
        };

        let rx = |x: f64| remap(x, old_minx, old_maxx, new_minx, new_maxx);
        let ry = |y: f64| remap(y, old_miny, old_maxy, new_miny, new_maxy);

        match self {
            Geometry::Stroke(pts) => {
                Geometry::Stroke(pts.iter().map(|&(x, y)| (rx(x), ry(y))).collect())
            }
            Geometry::Line {
                start: (x0, y0),
                end: (x1, y1),
            } => Geometry::Line {
                start: (rx(*x0), ry(*y0)),
                end: (rx(*x1), ry(*y1)),
            },
            Geometry::Arrow {
                start: (x0, y0),
                end: (x1, y1),
            } => Geometry::Arrow {
                start: (rx(*x0), ry(*y0)),
                end: (rx(*x1), ry(*y1)),
            },
            Geometry::Rect {
                origin: (ox, oy),
                size: (w, h),
            } => {
                let x0 = rx(*ox);
                let y0 = ry(*oy);
                let x1 = rx(*ox + *w);
                let y1 = ry(*oy + *h);
                Geometry::Rect {
                    origin: (x0.min(x1), y0.min(y1)),
                    size: ((x1 - x0).abs(), (y1 - y0).abs()),
                }
            }
            Geometry::Oval {
                origin: (ox, oy),
                size: (w, h),
            } => {
                let x0 = rx(*ox);
                let y0 = ry(*oy);
                let x1 = rx(*ox + *w);
                let y1 = ry(*oy + *h);
                Geometry::Oval {
                    origin: (x0.min(x1), y0.min(y1)),
                    size: ((x1 - x0).abs(), (y1 - y0).abs()),
                }
            }
            other => other.clone(),
        }
    }
}

/// Expand an AABB (minx, miny, maxx, maxy) to include the given primitive's
/// world-space bounds (transform.position is applied).
pub fn expand_aabb(
    (mut minx, mut miny, mut maxx, mut maxy): (f64, f64, f64, f64),
    prim: &super::Primitive,
) -> (f64, f64, f64, f64) {
    let (tx, ty) = prim.transform.position;
    match &prim.geometry {
        Geometry::Rect {
            origin: (ox, oy),
            size: (w, h),
        }
        | Geometry::Oval {
            origin: (ox, oy),
            size: (w, h),
        } => {
            minx = minx.min(tx + ox);
            miny = miny.min(ty + oy);
            maxx = maxx.max(tx + ox + w);
            maxy = maxy.max(ty + oy + h);
        }
        Geometry::Line {
            start: (x0, y0),
            end: (x1, y1),
        }
        | Geometry::Arrow {
            start: (x0, y0),
            end: (x1, y1),
        } => {
            minx = minx.min(tx + x0).min(tx + x1);
            miny = miny.min(ty + y0).min(ty + y1);
            maxx = maxx.max(tx + x0).max(tx + x1);
            maxy = maxy.max(ty + y0).max(ty + y1);
        }
        Geometry::Stroke(pts) => {
            for &(x, y) in pts {
                minx = minx.min(tx + x);
                miny = miny.min(ty + y);
                maxx = maxx.max(tx + x);
                maxy = maxy.max(ty + y);
            }
        }
        _ => {}
    }
    (minx, miny, maxx, maxy)
}

pub fn primitives_aabb<'a>(
    prims: impl Iterator<Item = &'a super::Primitive>,
) -> Option<(f64, f64, f64, f64)> {
    let init = (
        f64::INFINITY,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NEG_INFINITY,
    );
    let bb = prims.fold(init, |acc, p| expand_aabb(acc, p));
    if bb.0.is_infinite() { None } else { Some(bb) }
}
