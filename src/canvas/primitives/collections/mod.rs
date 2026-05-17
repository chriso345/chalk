use std::collections::HashMap;

use crate::canvas::primitives::Primitive;
use crate::canvas::primitives::collections::grid::GridCollection;
use crate::ui::components::config_popup::ConfigField;

pub mod grid;

#[derive(Clone, Debug, Copy)]
pub enum Collection {
    Grid { grid: grid::GridCollection },
    Other, // Placeholder for future collection types
}

impl Collection {
    pub fn generate(&self) -> Vec<Primitive> {
        match self {
            Collection::Grid { grid } => grid.generate(),
            _ => Vec::new(),
        }
    }
}

pub trait Configurable: Sized {
    fn config_fields(&self) -> Vec<ConfigField>;
    fn from_config(fields: &HashMap<String, String>) -> Option<Self>;
}

impl Collection {
    pub fn config_fields(&self) -> Vec<ConfigField> {
        match self {
            Collection::Grid { grid } => grid.config_fields(),
            Collection::Other => vec![],
        }
    }

    pub fn from_config(&self, fields: &HashMap<String, String>) -> Option<Collection> {
        match self {
            Collection::Grid { .. } => Some(Collection::Grid {
                grid: GridCollection::from_config(fields)?,
            }),
            Collection::Other => None,
        }
    }
}
