use std::sync::Arc;

use leptos::reactive::traits::Get;

use crate::{
    icon,
    ui::{
        layout::{Activation, Anchor, BoxConfig, Direction, Label, PanelConfig},
        overlay::OverlayContext,
    },
};

pub fn build_layout() -> Vec<PanelConfig> {
    vec![
        tool_picker_panel(),
        picker_panel(),
        zoom_corner(),
        undo_redo_panel(),
        dark_toggle(),
    ]
}

fn picker_panel() -> PanelConfig {
    let mut panel = PanelConfig::new("picker-panel", Anchor::CenterLeft)
        .offset(20, 0)
        .direction(Direction::Column)
        .gap(4)
        .padding(6);

    for entry in super::color::COLORS {
        let id = Box::leak(format!("color-{}", entry.name).into_boxed_str());
        let action = Box::leak(format!("action:set-color-{}", entry.name).into_boxed_str());
        panel = panel.add(BoxConfig::swatch(id, entry.color.to_hex(), action));
    }

    panel
        .add(BoxConfig::divider())
        .add(BoxConfig::stroke_width(
            "stroke-2",
            2,
            "action:set-stroke-2",
        ))
        .add(BoxConfig::stroke_width(
            "stroke-4",
            4,
            "action:set-stroke-4",
        ))
        .add(BoxConfig::stroke_width(
            "stroke-8",
            8,
            "action:set-stroke-8",
        ))
}

fn tool_picker_panel() -> PanelConfig {
    PanelConfig::new("toolbar", Anchor::TopCenter)
        .offset(0, 16)
        .direction(Direction::Row)
        .gap(2)
        .padding(6)
        .add(
            BoxConfig::icon_button(
                "tool-lock",
                "tool-lock-icon",
                icon!("lock.svg"),
                "action:lock-tool",
            )
            .with_hint("q"),
        )
        .add(
            BoxConfig::icon_button("tool-pan", "tool-pan-icon", icon!("pan.svg"), "tool:pan")
                .with_hint("v"),
        )
        .add(BoxConfig::icon_button(
            "tool-pointer",
            "tool-pointer-icon",
            icon!("pointer.svg"),
            "tool:pointer",
        ))
        .add(
            BoxConfig::icon_button("tool-pen", "tool-pen-icon", icon!("pencil.svg"), "tool:pen")
                .with_hint("p"),
        )
        .add(BoxConfig::divider())
        .add(
            BoxConfig::icon_button(
                "tool-line",
                "tool-line-icon",
                icon!("line.svg"),
                "tool:line",
            )
            .with_hint("l"),
        )
        .add(
            BoxConfig::icon_button(
                "tool-arrow",
                "tool-arrow-icon",
                icon!("arrow.svg"),
                "tool:arrow",
            )
            .with_hint("a"),
        )
        .add(
            BoxConfig::icon_button(
                "tool-rect",
                "tool-rect-icon",
                icon!("square.svg"),
                "tool:rect",
            )
            .with_hint("r"),
        )
        .add(
            BoxConfig::icon_button(
                "tool-circle",
                "tool-circle-icon",
                icon!("circle.svg"),
                "tool:circle",
            )
            .with_hint("c"),
        )
}

fn zoom_corner() -> PanelConfig {
    PanelConfig::new("zoom-badge", Anchor::BottomRight)
        .offset(-20, -20)
        .direction(Direction::Row)
        .gap(0)
        .padding(5)
        .add(BoxConfig::icon_button(
            "zoom-out",
            "zoom-out-icon",
            icon!("minus.svg"),
            "action:zoom-out",
        ))
        .add(BoxConfig::button(
            "zoom-pct",
            Label::Dynamic(Arc::new(move |ctx: &OverlayContext| {
                let zoom_pct = ctx.signals.zoom.read_only().get();
                format!("{zoom_pct}%")
            })),
            "action:reset-zoom",
        ))
        .add(BoxConfig::icon_button(
            "zoom-in",
            "zoom-in-icon",
            icon!("plus.svg"),
            "action:zoom-in",
        ))
}

fn undo_redo_panel() -> PanelConfig {
    PanelConfig::new("undo-redo", Anchor::BottomLeft)
        .offset(20, -20)
        .direction(Direction::Row)
        .gap(4)
        .padding(6)
        .add(BoxConfig::icon_button(
            "action-undo",
            "action-undo-icon",
            icon!("undo.svg"),
            "action:undo",
        ))
        .add(BoxConfig::divider())
        .add(BoxConfig::icon_button(
            "action-redo",
            "action-redo-icon",
            icon!("redo.svg"),
            "action:redo",
        ))
        .add(BoxConfig::divider())
        .add(BoxConfig::icon_button(
            "action-clear",
            "action-clear-icon",
            icon!("trash.svg"),
            "action:clear",
        ))
}

fn dark_toggle() -> PanelConfig {
    PanelConfig::new("dark-mode-toggle", Anchor::TopRight)
        .offset(-20, 20)
        .direction(Direction::Row)
        .gap(0)
        .padding(6)
        .add(BoxConfig::icon_dropdown(
            Activation::Click,
            "background-dropdown",
            "background-dropdown-icon",
            icon!("background.svg"),
            background_dropdown_items(),
        ))
        .add(BoxConfig::icon_button(
            "toggle-dark-mode",
            "toggle-dark-mode-icon",
            icon!("moon.svg"),
            "action:toggle-dark-mode",
        ))
}

fn background_dropdown_items() -> Vec<BoxConfig> {
    vec![
        BoxConfig::icon_button(
            "set-canvas-dots",
            "set-canvas-dots-icon",
            icon!("grain.svg"),
            "action:set-canvas-dots",
        ),
        BoxConfig::icon_button(
            "set-canvas-grid",
            "set-canvas-grid-icon",
            icon!("grid.svg"),
            "action:set-canvas-grid",
        ),
        BoxConfig::icon_button(
            "set-canvas-none",
            "set-canvas-none-icon",
            icon!("none.svg"),
            "action:set-canvas-none",
        ),
    ]
}
