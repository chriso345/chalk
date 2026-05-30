use std::collections::HashMap;

use crate::canvas::primitives::Primitive;
use crate::canvas::primitives::collections::grid::GridCollection;
use crate::ui::components::config_popup::ConfigField;

pub mod grid;
pub mod network;

#[derive(Clone, Debug, Copy)]
pub enum Collection {
    Grid { grid: grid::GridCollection },
    Network { network: network::NetworkCollection },
    Other, // Placeholder for future collection types
}

impl Collection {
    pub fn generate(&self) -> Vec<Primitive> {
        match self {
            Collection::Grid { grid } => grid.generate(),
            Collection::Network { network } => network.generate(),
            Collection::Other => vec![], // No primitives for unknown collection
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
            Collection::Network { network } => network.config_fields(),
            Collection::Other => vec![],
        }
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn from_config(&self, fields: &HashMap<String, String>) -> Option<Collection> {
        match self {
            Collection::Grid { .. } => Some(Collection::Grid {
                grid: GridCollection::from_config(fields)?,
            }),
            Collection::Network { .. } => Some(Collection::Network {
                network: network::NetworkCollection::from_config(fields)?,
            }),
            Collection::Other => None,
        }
    }
}
