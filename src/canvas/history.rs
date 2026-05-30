use crate::canvas::{action::ChalkAction, primitives::Primitive};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct History {
    /// Linear history of actions
    actions: VecDeque<ChalkAction>,
    /// How many actions are "active"
    cursor: usize,
    /// Capacity of the history; when exceeded, oldest actions are dropped
    capacity: usize,
}

impl Default for History {
    fn default() -> Self {
        Self {
            actions: VecDeque::new(),
            cursor: 0,
            capacity: 100,
        }
    }
}

impl History {
    pub fn apply(&mut self, doc: &mut Vec<Primitive>, action: ChalkAction) {
        while self.actions.len() > self.capacity {
            self.actions.pop_front();

            if self.cursor > 0 {
                self.cursor -= 1;
            }
        }

        if self.cursor < self.actions.len() {
            self.actions.truncate(self.cursor);
        }

        self.apply_action(doc, &action);
        self.actions.push_back(action);
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
        if self.cursor < self.actions.len() {
            self.actions.truncate(self.cursor);
        }

        if self.actions.len() == self.capacity {
            self.actions.pop_front();

            if self.cursor > 0 {
                self.cursor -= 1;
            }
        }

        self.actions.push_back(action);
        self.cursor += 1;
    }
}
