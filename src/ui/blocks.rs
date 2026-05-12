use std::sync::Arc;

use leptos::reactive::traits::Get;

use crate::ui::{
    layout::{Anchor, BoxConfig, Direction, Label, PanelConfig},
    overlay::OverlayContext,
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
    PanelConfig::new("picker-panel", Anchor::CenterLeft)
        .offset(20, 0)
        .direction(Direction::Column)
        .gap(4)
        .padding(6)
        .add(BoxConfig::swatch(
            "color-white",
            "#F2F0EF",
            "action:set-color-white",
        ))
        .add(BoxConfig::swatch(
            "color-black",
            "#1A1A18",
            "action:set-color-black",
        ))
        .add(BoxConfig::swatch(
            "color-red",
            "#E24B4A",
            "action:set-color-red",
        ))
        .add(BoxConfig::swatch(
            "color-green",
            "#4AB557",
            "action:set-color-green",
        ))
        .add(BoxConfig::swatch(
            "color-blue",
            "#378ADD",
            "action:set-color-blue",
        ))
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
                "/public/icons/lock.svg",
                "action:lock-tool",
            )
            .with_hint("q"),
        )
        .add(
            BoxConfig::icon_button(
                "tool-pan",
                "tool-pan-icon",
                "/public/icons/pan.svg",
                "tool:pan",
            )
            .with_hint("v"),
        )
        .add(BoxConfig::icon_button(
            "tool-pointer",
            "tool-pointer-icon",
            "/public/icons/pointer.svg",
            "tool:pointer",
        ))
        .add(
            BoxConfig::icon_button(
                "tool-pen",
                "tool-pen-icon",
                "/public/icons/pencil.svg",
                "tool:pen",
            )
            .with_hint("p"),
        )
        .add(BoxConfig::divider())
        .add(
            BoxConfig::icon_button(
                "tool-line",
                "tool-line-icon",
                "/public/icons/line.svg",
                "tool:line",
            )
            .with_hint("l"),
        )
        .add(
            BoxConfig::icon_button(
                "tool-arrow",
                "tool-arrow-icon",
                "/public/icons/arrow.svg",
                "tool:arrow",
            )
            .with_hint("a"),
        )
        .add(
            BoxConfig::icon_button(
                "tool-rect",
                "tool-rect-icon",
                "/public/icons/square.svg",
                "tool:rect",
            )
            .with_hint("r"),
        )
        .add(
            BoxConfig::icon_button(
                "tool-circle",
                "tool-circle-icon",
                "/public/icons/circle.svg",
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
            "/public/icons/minus.svg",
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
            "/public/icons/plus.svg",
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
            "/public/icons/undo.svg",
            "action:undo",
        ))
        .add(BoxConfig::divider())
        .add(BoxConfig::icon_button(
            "action-redo",
            "action-redo-icon",
            "/public/icons/redo.svg",
            "action:redo",
        ))
        .add(BoxConfig::divider())
        .add(BoxConfig::icon_button(
            "action-clear",
            "action-clear-icon",
            "/public/icons/trash.svg",
            "action:clear",
        ))
}

fn dark_toggle() -> PanelConfig {
    PanelConfig::new("dark-mode-toggle", Anchor::TopRight)
        .offset(-20, 20)
        .direction(Direction::Row)
        .gap(0)
        .padding(6)
        .add(BoxConfig::icon_button(
            "toggle-dark-mode",
            "toggle-dark-mode-icon",
            "/public/icons/moon.svg",
            "action:toggle-dark-mode",
        ))
}
