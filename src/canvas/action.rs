use crate::canvas::primitives::Primitive;

#[derive(Clone, Debug)]
pub enum ChalkAction {
    Add {
        primitive: Primitive,
    },
    Delete {
        primitive: Primitive,
        index: usize,
    },
    Transform {
        before: Primitive,
        after: Primitive,
        index: usize,
    },
    Clear {
        previous: Vec<Primitive>,
    },
    Batch {
        actions: Vec<ChalkAction>,
    },
}

impl ChalkAction {
    pub fn len(&self) -> usize {
        match self {
            ChalkAction::Add { .. }
            | ChalkAction::Delete { .. }
            | ChalkAction::Transform { .. } => 1,
            ChalkAction::Clear { previous } => previous.len(),
            ChalkAction::Batch { actions } => actions.iter().map(|a| a.len()).sum(),
        }
    }
}
