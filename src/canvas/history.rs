use crate::canvas::{action::ChalkAction, primitives::Primitive};

#[derive(Clone, Debug, Default)]
pub struct History {
    /// Linear history of actions
    actions: Vec<ChalkAction>,
    /// How many actions are "active"
    cursor: usize,
}

impl History {
    pub fn apply(&mut self, doc: &mut Vec<Primitive>, action: ChalkAction) {
        // discard redo stack
        self.actions.truncate(self.cursor);

        self.apply_action(doc, &action);
        self.actions.push(action);
        self.cursor += 1;
    }

    fn apply_action(&self, doc: &mut Vec<Primitive>, action: &ChalkAction) {
        match action {
            ChalkAction::Add { primitive } => {
                doc.push(primitive.clone());
            }
            ChalkAction::Delete { index, .. } => {
                doc.remove(*index);
            }
            ChalkAction::Transform { index, after, .. } => {
                doc[*index] = after.clone();
            }
            ChalkAction::Clear { .. } => {
                doc.clear();
            }
            ChalkAction::Batch { actions } => {
                for action in actions {
                    self.apply_action(doc, action);
                }
            }
        }
    }

    pub fn undo(&mut self, doc: &mut Vec<Primitive>) {
        if self.cursor == 0 {
            return;
        }

        self.cursor -= 1;
        let action = &self.actions[self.cursor];
        self.undo_action(doc, action);
    }

    fn undo_action(&self, doc: &mut Vec<Primitive>, action: &ChalkAction) {
        match action {
            ChalkAction::Add { .. } => {
                doc.pop();
            }
            ChalkAction::Delete { primitive, index } => {
                doc.insert(*index, primitive.clone());
            }
            ChalkAction::Transform { index, before, .. } => {
                doc[*index] = before.clone();
            }
            ChalkAction::Clear { previous } => {
                *doc = previous.clone();
            }
            ChalkAction::Batch { actions } => {
                for a in actions.iter().rev() {
                    self.undo_action(doc, a);
                }
            }
        }
    }

    pub fn redo(&mut self, doc: &mut Vec<Primitive>) {
        if self.cursor >= self.actions.len() {
            return;
        }

        let action = &self.actions[self.cursor];
        self.apply_action(doc, action);
        self.cursor += 1;
    }

    pub fn clear(&mut self, doc: &mut Vec<Primitive>) {
        self.apply(
            doc,
            ChalkAction::Clear {
                previous: doc.clone(),
            },
        );
    }

    pub fn push_without_apply(&mut self, action: ChalkAction) {
        self.actions.truncate(self.cursor);
        self.actions.push(action);
        self.cursor += 1;
    }
}
