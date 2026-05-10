use crate::canvas::types::Point;

/// The kind of shape tool currently selected.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShapeKind {
    Line,
    Arrow,
    Rect,
    Oval,
}

/// A fully-specified geometric shape stored in history.
#[derive(Clone, Debug)]
pub enum Shape {
    Line { start: Point, end: Point },
    Arrow { start: Point, end: Point },
    Rect { origin: Point, size: (f64, f64) },
    // Circle { center: Point, radius: f64 },
    Oval { origin: Point, size: (f64, f64) },
}

/// Transient state for a shape being drawn. Stores anchor + live cursor so
/// the controller can call `build` on every move event.
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

    pub fn build(&self) -> Shape {
        let (ax, ay) = self.anchor;
        let (cx, cy) = self.current;
        match self.kind {
            ShapeKind::Line => Shape::Line {
                start: self.anchor,
                end: self.current,
            },
            ShapeKind::Arrow => Shape::Arrow {
                start: self.anchor,
                end: self.current,
            },
            ShapeKind::Rect => Shape::Rect {
                origin: (ax.min(cx), ay.min(cy)),
                size: ((cx - ax).abs(), (cy - ay).abs()),
            },
            ShapeKind::Oval => Shape::Oval {
                origin: (ax.min(cx), ay.min(cy)),
                size: ((cx - ax).abs(), (cy - ay).abs()),
            },
        }
    }
}
