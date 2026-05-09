use std::sync::Arc;

use leptos::prelude::*;

use crate::canvas::whiteboard::Whiteboard;
use crate::signals::ChalkSignals;
use crate::ui::layout::{Anchor, BoxConfig, Direction, Label, PanelConfig};
use crate::ui::overlay::{Overlay, OverlayContext};

fn build_layout() -> Vec<PanelConfig> {
    vec![
        // Zoom badge - bottom right
        PanelConfig::new("zoom-badge", Anchor::BottomRight)
            .offset(-20, -20)
            .direction(Direction::Row)
            .gap(0)
            .padding(5)
            // .add(BoxConfig::badge("zoom-pct", "zoom")),
            .add(BoxConfig::button(
                "zoom-pct",
                Label::Dynamic(Arc::new(move |ctx: &OverlayContext| {
                    let zoom_pct = ctx.signals.zoom.read_only().get();
                    format!("Zoom: {zoom_pct}%")
                })),
                "action:reset-zoom",
            )),
        // Undo-redo menu - bottom left
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
    let signals = ChalkSignals::new();

    let on_action = Callback::new(move |action: &'static str| {
        // TODO: Change action to a proper action type
        leptos::logging::log!("[action] {action}");
        match action {
            "tool:draw" => {}
            "action:undo" => {
                signals.undo.update(|n| *n += 1);
            }
            "action:redo" => {
                signals.redo.update(|n| *n += 1);
            }
            "action:clear" => {
                signals.clear.update(|n| *n += 1);
            }
            "action:reset-zoom" => {
                signals.zoom.set(100);
            }
            "action:toggle-dark-mode" => {
                signals.dark_mode.update(|b| *b = !*b);
            }
            _ => {}
        }
    });

    let ctx = OverlayContext { signals, on_action };
    let panels = build_layout();

    view! {
        <Whiteboard signals=signals />
        <Overlay panels=panels ctx=ctx />
    }
}
