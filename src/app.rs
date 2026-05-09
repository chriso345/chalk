use std::sync::Arc;

use leptos::prelude::*;

use crate::canvas::{primitives::ShapeKind, tool::Tool, whiteboard::Whiteboard};
use crate::signals::ChalkSignals;
use crate::ui::components::palette::Palette;
use crate::ui::keybindings;
use crate::ui::layout::{Anchor, BoxConfig, Direction, Label, PanelConfig};
use crate::ui::overlay::{Overlay, OverlayContext};
use crate::utils::on_mount;

fn build_layout() -> Vec<PanelConfig> {
    vec![
        // Tool picker - top center
        PanelConfig::new("toolbar", Anchor::TopCenter)
            .offset(0, 16)
            .direction(Direction::Row)
            .gap(2)
            .padding(6)
            .add(BoxConfig::icon_button(
                "tool-pan",
                "/public/icons/pan.svg",
                "tool:pan",
            ))
            .add(BoxConfig::icon_button(
                "tool-pen",
                "/public/icons/pencil.svg",
                "tool:pen",
            ))
            .add(BoxConfig::divider())
            .add(BoxConfig::icon_button(
                "tool-line",
                "/public/icons/line.svg",
                "tool:line",
            ))
            .add(BoxConfig::icon_button(
                "tool-arrow",
                "/public/icons/arrow.svg",
                "tool:arrow",
            ))
            .add(BoxConfig::icon_button(
                "tool-rect",
                "/public/icons/square.svg",
                "tool:rect",
            ))
            .add(BoxConfig::icon_button(
                "tool-circle",
                "/public/icons/circle.svg",
                "tool:circle",
            )),
        // Zoom badge - bottom right
        PanelConfig::new("zoom-badge", Anchor::BottomRight)
            .offset(-20, -20)
            .direction(Direction::Row)
            .gap(0)
            .padding(5)
            .add(BoxConfig::icon_button(
                "zoom-out",
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
                "/public/icons/plus.svg",
                "action:zoom-in",
            )),
        // Undo / redo / clear - bottom left
        PanelConfig::new("undo-redo", Anchor::BottomLeft)
            .offset(20, -20)
            .direction(Direction::Row)
            .gap(4)
            .padding(6)
            .add(BoxConfig::icon_button(
                "action-undo",
                "/public/icons/undo.svg",
                "action:undo",
            ))
            .add(BoxConfig::divider())
            .add(BoxConfig::icon_button(
                "action-redo",
                "/public/icons/redo.svg",
                "action:redo",
            ))
            .add(BoxConfig::divider())
            .add(BoxConfig::icon_button(
                "action-clear",
                "/public/icons/trash.svg",
                "action:clear",
            )),
        // Dark mode toggle - top right
        PanelConfig::new("dark-mode-toggle", Anchor::TopRight)
            .offset(-20, 20)
            .direction(Direction::Row)
            .gap(0)
            .padding(6)
            .add(BoxConfig::icon_button(
                "action-toggle-dark-mode",
                "/public/icons/moon.svg",
                "action:toggle-dark-mode",
            )),
    ]
}

#[component]
pub fn App() -> impl IntoView {
    on_mount();

    let signals = ChalkSignals::new();

    let on_action = Callback::new(move |action: &'static str| {
        // TODO: Change action to a proper action type (this can then be stored in history for undo/redo)
        match action {
            "tool:pan" => signals.tool.set(Tool::Pan),
            "tool:pen" => signals.tool.set(Tool::Pen),
            "tool:line" => signals.tool.set(Tool::Shape(ShapeKind::Line)),
            "tool:arrow" => signals.tool.set(Tool::Shape(ShapeKind::Arrow)),
            "tool:rect" => signals.tool.set(Tool::Shape(ShapeKind::Rect)),
            "tool:circle" => signals.tool.set(Tool::Shape(ShapeKind::Circle)),

            "action:undo" => signals.undo.update(|n| *n += 1),
            "action:redo" => signals.redo.update(|n| *n += 1),
            "action:clear" => signals.clear.update(|n| *n += 1),

            "action:reset-zoom" => signals.zoom.set(100),
            "action:zoom-in" => signals.zoom.update(|n| *n = (*n + 10).min(3000)),
            "action:zoom-out" => signals.zoom.update(|n| *n = (*n - 10).max(10)),

            "action:toggle-dark-mode" => signals.dark_mode.update(|b| *b = !*b),
            "ui:open-palette" => signals.palette_open.update(|v| *v = !*v),

            _ => {
                leptos::logging::log!("Unknown action: {action}");
            }
        }
    });

    Effect::new({
        let on_action = on_action.clone();
        move |_| {
            keybindings::register(on_action);
        }
    });

    let ctx = OverlayContext { signals, on_action };
    let panels = build_layout();

    view! {
        <Whiteboard signals=signals />
        <Overlay panels=panels ctx=ctx />

        <Palette open=signals.palette_open on_action=on_action />
    }
}
