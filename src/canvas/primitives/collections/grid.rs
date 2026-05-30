use crate::{
    canvas::primitives::{Geometry, Primitive, PrimitiveStyle, collections::Configurable},
    ui::components::config_popup::ConfigField,
};
use std::collections::HashMap;

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
            ConfigField::Float2 {
                key_a: "origin_x",
                key_b: "origin_y",
                label: "Origin",
                label_a: "x",
                label_b: "y",
                default_a: self.origin.0,
                default_b: self.origin.1,
            },
            ConfigField::Float2 {
                key_a: "spacing_x",
                key_b: "spacing_y",
                label: "Spacing",
                label_a: "x",
                label_b: "y",
                default_a: self.spacing.0,
                default_b: self.spacing.1,
            },
            ConfigField::Int2 {
                key_a: "count_x",
                key_b: "count_y",
                label: "Count",
                label_a: "cols",
                label_b: "rows",
                default_a: self.count.0 as i64,
                default_b: self.count.1 as i64,
            },
        ]
    }

    fn from_config(fields: &HashMap<String, String>) -> Option<Self> {
        let f = |k: &str| fields.get(k)?.trim().parse::<f64>().ok();
        let u = |k: &str| fields.get(k)?.trim().parse::<u32>().ok();

        Some(GridCollection {
            origin: (f("origin_x")?, f("origin_y")?),
            spacing: (f("spacing_x")?, f("spacing_y")?),
            count: (u("count_x")?, u("count_y")?),
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
