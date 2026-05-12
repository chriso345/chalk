use crate::canvas::{primitives::geometry::Geometry, types::Point};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShapeKind {
    Line,
    Arrow,
    Rect,
    Oval,
}

/// Transient in-progress shape being drawn. Produces `Geometry` on `build()`.
#[derive(Clone, Debug)]
pub struct ShapeInProgress {
    pub kind: ShapeKind,
    pub anchor: Point,
    pub current: Point,
}

impl ShapeInProgress {
    pub fn new(kind: ShapeKind, anchor: Point) -> Self {
        Self {
            kind,
            anchor,
            current: anchor,
        }
    }

    pub fn update(&mut self, current: Point, snap: bool) {
        if !snap {
            self.current = current;
            return;
        }
        match self.kind {
            ShapeKind::Rect | ShapeKind::Oval => {
                let (ax, ay) = self.anchor;
                let (cx, cy) = current;
                let dx = cx - ax;
                let dy = cy - ay;
                let size = dx.abs().max(dy.abs());
                self.current = (ax + size.copysign(dx), ay + size.copysign(dy));
            }
            _ => {
                self.current = current;
            }
        }
    }

    pub fn build(&self) -> Geometry {
        let (ax, ay) = self.anchor;
        let (cx, cy) = self.current;
        match self.kind {
            ShapeKind::Line => Geometry::Line {
                start: self.anchor,
                end: self.current,
            },
            ShapeKind::Arrow => Geometry::Arrow {
                start: self.anchor,
                end: self.current,
            },
            ShapeKind::Rect => Geometry::Rect {
                origin: (ax.min(cx), ay.min(cy)),
                size: ((cx - ax).abs(), (cy - ay).abs()),
            },
            ShapeKind::Oval => Geometry::Oval {
                origin: (ax.min(cx), ay.min(cy)),
                size: ((cx - ax).abs(), (cy - ay).abs()),
            },
        }
    }
}
