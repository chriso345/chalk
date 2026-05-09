use crate::canvas::primitives::Primitive;

#[derive(Clone, Debug, Default)]
pub struct History {
    /// All committed primitives
    primitives: Vec<Primitive>,
    /// How many strokes are currently "active" (the rest are undone)
    cursor: usize,
}

impl History {
    pub fn push(&mut self, primitive: Primitive) {
        // Pushing a new shape discards any redoable future
        self.primitives.truncate(self.cursor);
        self.primitives.push(primitive);
        self.cursor = self.primitives.len();
    }

    pub fn undo(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn redo(&mut self) {
        if self.cursor < self.primitives.len() {
            self.cursor += 1;
        }
    }

    pub fn clear(&mut self) {
        self.primitives.clear();
        self.cursor = 0;
    }

    pub fn visible(&self) -> &[Primitive] {
        &self.primitives[..self.cursor]
    }

    pub fn can_undo(&self) -> bool {
        self.cursor > 0
    }

    pub fn can_redo(&self) -> bool {
        self.cursor < self.primitives.len()
    }
}
