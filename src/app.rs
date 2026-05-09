use leptos::prelude::*;

use crate::canvas::whiteboard::Whiteboard;
use crate::ui::layout::{Anchor, BoxConfig, Direction, PanelConfig};
use crate::ui::overlay::{Overlay, OverlayContext};

fn build_layout() -> Vec<PanelConfig> {
    vec![
        // Toolbar - top center
        PanelConfig::new("toolbar", Anchor::TopCenter)
            .offset(0, 16)
            .direction(Direction::Row)
            .gap(2)
            .padding(6)
            .add(BoxConfig::button("action-clear", "clear", "action:clear")),
        // Zoom badge - bottom right
        PanelConfig::new("zoom-badge", Anchor::BottomRight)
            .offset(-20, -20)
            .direction(Direction::Row)
            .gap(0)
            .padding(5)
            .add(BoxConfig::badge("zoom-pct", "zoom")),
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
            .add(BoxConfig::icon_button(
                "action-redo",
                "/public/icons/redo.svg",
                "action:redo",
            )),
    ]
}

#[component]
pub fn App() -> impl IntoView {
    // TODO: Use a special type for combined signals -> let zoom_ = RWSignal::new(100_u32).read_only(); etc...
    let (zoom_pct, set_zoom_pct) = signal(100_u32);
    let (should_clear, set_should_clear) = signal(0_u32);

    let on_action = Callback::new(move |action: &'static str| {
        // TODO: Change action to a proper action type
        leptos::logging::log!("[action] {action}");
        match action {
            "tool:draw" => {}
            "action:undo" => {
                leptos::logging::log!("undo!");
            }
            "action:redo" => {
                leptos::logging::log!("redo!");
            }
            "action:clear" => {
                set_should_clear.update(|n| *n += 1);
            }
            _ => {}
        }
    });

    let panels = build_layout();
    let ctx = OverlayContext {
        zoom_pct,
        on_action,
    };

    view! {
        <Whiteboard set_zoom_pct=set_zoom_pct should_clear=should_clear/>
        <Overlay panels=panels ctx=ctx />
    }
}
