use crate::{
    canvas::primitives::{Geometry, Primitive, PrimitiveStyle, collections::Configurable},
    ui::components::config_popup::ConfigField,
};
use std::collections::HashMap;

#[derive(Clone, Debug, Copy)]
pub struct NetworkCollection {
    pub origin: (f64, f64),
    pub number_of_nodes: u32,
    pub directed: bool,
    pub node_spacing: f64,
    pub node_size: f64,
    pub arc_probability: f64, // Chance of an edge existing between any two nodes (0.0 to 1.0)
}

impl NetworkCollection {
    pub fn new(
        origin: (f64, f64),
        number_of_nodes: u32,
        directed: bool,
        node_spacing: f64,
        node_size: f64,
        arc_probability: f64,
    ) -> Self {
        Self {
            origin,
            number_of_nodes,
            directed,
            node_spacing,
            node_size,
            arc_probability,
        }
    }

    fn node_position(&self, i: u32) -> (f64, f64) {
        let (ox, oy) = self.origin;

        // simple circular layout (good enough for non-perfect placement)
        let n = self.number_of_nodes.max(1) as f64;
        let angle = (i as f64 / n) * std::f64::consts::TAU;

        let radius = self.node_spacing * (n.sqrt().max(1.0));

        (ox + radius * angle.cos(), oy + radius * angle.sin())
    }

    pub fn generate(&self) -> Vec<Primitive> {
        let mut primitives = Vec::new();
        let style = PrimitiveStyle::default();

        let n = self.number_of_nodes;

        let mut positions = Vec::with_capacity(n as usize);

        for i in 0..n {
            let pos = self.node_position(i);
            positions.push(pos);

            primitives.push(Primitive::new(
                Geometry::Oval {
                    origin: (pos.0 - self.node_size / 2.0, pos.1 - self.node_size / 2.0),
                    size: (self.node_size, self.node_size),
                },
                style.clone(),
            ));
        }

        for i in 0..n {
            for j in (i + 1)..n {
                // Roll the dice to decide if we should create an edge between nodes i and j
                if rand::random::<f64>() >= self.arc_probability {
                    continue;
                }

                let (x1, y1) = positions[i as usize];
                let (x2, y2) = positions[j as usize];

                let dx = x2 - x1;
                let dy = y2 - y1;

                let dist = (dx * dx + dy * dy).sqrt();

                if dist == 0.0 {
                    continue;
                }

                let offset = self.node_size / 2.0;

                let ux = dx / dist;
                let uy = dy / dist;

                let start = (x1 + ux * offset, y1 + uy * offset);
                let end = (x2 - ux * offset, y2 - uy * offset);

                primitives.push(Primitive::new(
                    if self.directed {
                        Geometry::Arrow { start, end }
                    } else {
                        Geometry::Line { start, end }
                    },
                    style.clone(),
                ));
            }
        }

        primitives
    }
}

impl Configurable for NetworkCollection {
    fn config_fields(&self) -> Vec<ConfigField> {
        vec![
            ConfigField::Float2 {
                key_a: "origin_x",
                key_b: "origin_y",
                label: "Origin",
                label_a: "x",
                label_b: "y",
                default_a: self.origin.0,
                default_b: self.origin.1,
            },
            ConfigField::Int {
                key: "number_of_nodes",
                label: "Nodes",
                default: self.number_of_nodes as i64,
            },
            ConfigField::Bool {
                key: "directed",
                label: "Directed",
                default: self.directed,
            },
            ConfigField::Float {
                key: "node_spacing",
                label: "Node spacing",
                default: self.node_spacing,
            },
        ]
    }

    fn from_config(fields: &HashMap<String, String>) -> Option<Self> {
        let f = |k: &str| fields.get(k)?.trim().parse::<f64>().ok();
        let i = |k: &str| fields.get(k)?.trim().parse::<u32>().ok();
        let b = |k: &str| fields.get(k)?.trim().parse::<bool>().ok();

        Some(NetworkCollection {
            origin: (f("origin_x")?, f("origin_y")?),
            number_of_nodes: i("number_of_nodes")?,
            directed: b("directed")?,
            node_spacing: f("node_spacing")?,
            node_size: 50.0, // fixed size for now, could be made configurable if desired
            arc_probability: 0.5, // fixed probability for now, could be made configurable if desired
        })
    }
}

impl Default for NetworkCollection {
    fn default() -> Self {
        Self {
            origin: (0.0, 0.0),
            number_of_nodes: 5,
            directed: false,
            node_spacing: 40.0,
            node_size: 50.0,
            arc_probability: 0.5,
        }
    }
}
