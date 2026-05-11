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
    // TODO: Not Implemented: Event for moving and resizing shape(s)
    Transform {
        before: Primitive,
        after: Primitive,
        index: usize,
    },
    Clear {
        previous: Vec<Primitive>,
    },
}
