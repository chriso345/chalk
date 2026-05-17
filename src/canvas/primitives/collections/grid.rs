use std::collections::HashMap;

use crate::{
    canvas::primitives::{Geometry, Primitive, PrimitiveStyle, collections::Configurable},
    ui::components::config_popup::ConfigField,
};

#[derive(Clone, Debug, Copy)]
pub struct GridCollection {
    pub origin: (f64, f64),
    pub spacing: (f64, f64),
    pub count: (u32, u32),
}

impl GridCollection {
    pub fn new(origin: (f64, f64), spacing: (f64, f64), count: (u32, u32)) -> Self {
        GridCollection {
            origin,
            spacing,
            count,
        }
    }

    pub fn generate(&self) -> Vec<Primitive> {
        let mut primitives = Vec::new();

        let (ox, oy) = self.origin;
        let (sx, sy) = self.spacing;
        let (nx, ny) = self.count;

        // Optional: reuse one style
        let style = PrimitiveStyle::default();

        let width = (nx.saturating_sub(1)) as f64 * sx;
        let height = (ny.saturating_sub(1)) as f64 * sy;

        // Vertical lines
        for i in 0..nx {
            let x = ox + i as f64 * sx;

            primitives.push(Primitive::new(
                Geometry::Line {
                    start: (x, oy),
                    end: (x, oy + height),
                },
                style.clone(),
            ));
        }

        // Horizontal lines
        for j in 0..ny {
            let y = oy + j as f64 * sy;

            primitives.push(Primitive::new(
                Geometry::Line {
                    start: (ox, y),
                    end: (ox + width, y),
                },
                style.clone(),
            ));
        }

        primitives
    }
}

impl Configurable for GridCollection {
    fn config_fields(&self) -> Vec<ConfigField> {
        vec![
            ConfigField {
                name: "origin",
                label: "Origin (x, y)",
                value: format!("{},{}", self.origin.0, self.origin.1),
            },
            ConfigField {
                name: "spacing",
                label: "Spacing (x, y)",
                value: format!("{},{}", self.spacing.0, self.spacing.1),
            },
            ConfigField {
                name: "count",
                label: "Count (x, y)",
                value: format!("{},{}", self.count.0, self.count.1),
            },
        ]
    }

    fn from_config(fields: &HashMap<String, String>) -> Option<Self> {
        let parse_pair_f = |s: &str| -> Option<(f64, f64)> {
            let mut it = s.splitn(2, ',');
            Some((
                it.next()?.trim().parse().ok()?,
                it.next()?.trim().parse().ok()?,
            ))
        };
        let parse_pair_u = |s: &str| -> Option<(u32, u32)> {
            let mut it = s.splitn(2, ',');
            Some((
                it.next()?.trim().parse().ok()?,
                it.next()?.trim().parse().ok()?,
            ))
        };

        Some(GridCollection {
            origin: parse_pair_f(fields.get("origin")?)?,
            spacing: parse_pair_f(fields.get("spacing")?)?,
            count: parse_pair_u(fields.get("count")?)?,
        })
    }
}

impl Default for GridCollection {
    fn default() -> Self {
        GridCollection {
            origin: (0.0, 0.0),
            spacing: (50.0, 50.0),
            count: (10, 10),
        }
    }
}
