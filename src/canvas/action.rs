use crate::canvas::primitives::Primitive;

#[derive(Clone, Debug)]
pub enum ChalkAction {
    Add {
        primitive: Primitive,
    },
    // TODO: Not Implemented: Event for individial shape removal
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
