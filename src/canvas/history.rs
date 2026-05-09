use crate::canvas::types::Stroke;

#[derive(Clone, Debug, Default)]
pub struct History {
    /// All committed strokes
    strokes: Vec<Stroke>,
    /// How many strokes are currently "active" (the rest are undone)
    cursor: usize,
}

impl History {
    pub fn push(&mut self, stroke: Stroke) {
        // Pushing a new stroke discards any redoable future
        self.strokes.truncate(self.cursor);
        self.strokes.push(stroke);
        self.cursor = self.strokes.len();
    }

    pub fn undo(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn redo(&mut self) {
        if self.cursor < self.strokes.len() {
            self.cursor += 1;
        }
    }

    pub fn clear(&mut self) {
        self.strokes.clear();
        self.cursor = 0;
    }

    /// The slice of strokes that should currently be drawn
    pub fn visible(&self) -> &[Stroke] {
        &self.strokes[..self.cursor]
    }

    pub fn can_undo(&self) -> bool {
        self.cursor > 0
    }

    pub fn can_redo(&self) -> bool {
        self.cursor < self.strokes.len()
    }
}
