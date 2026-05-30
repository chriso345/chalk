use leptos::prelude::*;

use crate::canvas::background::BackgroundKind;
use crate::canvas::primitives::collections::Collection;
use crate::canvas::{primitives::ShapeKind, tool::Tool, whiteboard::Whiteboard};
use crate::signals::ChalkSignals;
use crate::ui::blocks::build_layout;
use crate::ui::color::ChalkColor;
use crate::ui::components::generation::GenerationFlow;
use crate::ui::components::palette::Palette;
use crate::ui::keybindings;
use crate::ui::overlay::{Overlay, OverlayContext};
use crate::utils::on_mount;

#[component]
pub fn App() -> impl IntoView {
    on_mount();

    let signals = ChalkSignals::new();

    let generation_palette_open = RwSignal::new(false);
    let generation_palette_open_cb = generation_palette_open;
    let on_action = Callback::new(move |action: &'static str| {
        // TODO: Change action to a proper action type (this can then be stored in history for undo/redo)
        match action {
            "tool:pan" => signals.tool.set(Tool::Pan),
            "tool:pointer" => signals.tool.set(Tool::Pointer),
            "tool:pen" => signals.tool.set(Tool::Pen),
            "tool:line" => signals.tool.set(Tool::Shape(ShapeKind::Line)),
            "tool:arrow" => signals.tool.set(Tool::Shape(ShapeKind::Arrow)),
            "tool:rect" => signals.tool.set(Tool::Shape(ShapeKind::Rect)),
            "tool:circle" => signals.tool.set(Tool::Shape(ShapeKind::Oval)),

            "action:undo" => signals.undo.update(|n| *n += 1),
            "action:redo" => signals.redo.update(|n| *n += 1),
            "action:clear" => signals.clear.update(|n| *n += 1),
            "action:delete-selection" => signals.delete_selection.update(|n| *n += 1),

            "action:reset-zoom" => signals.zoom.set(100),
            "action:zoom-in" => signals.zoom.update(|n| *n = (*n + 10).min(3000)),
            "action:zoom-out" => signals.zoom.update(|n| *n = (*n - 10).max(10)),

            "action:lock-tool" => signals.lock_tool.update(|b| *b = !*b),
            "action:toggle-dark-mode" => signals.dark_mode.update(|b| *b = !*b),
            "ui:open-palette" => signals.palette_open.update(|v| *v = !*v),
            "ui:open-generation-palette" => generation_palette_open_cb.update(|v| *v = !*v),

            action if action.starts_with("action:set-color-") => {
                let color = action.trim_start_matches("action:set-color-");
                let ccolor = ChalkColor::from_word(color);
                if let Some(col) = ccolor {
                    signals.color.set(col);
                }
            }

            action if action.starts_with("action:set-stroke-") => {
                let width = action.trim_start_matches("action:set-stroke-").parse();
                if let Ok(width) = width {
                    signals.stroke_width.set(width);
                }
            }

            action if action.starts_with("action:set-canvas-") => {
                let background = action.trim_start_matches("action:set-canvas-");
                let bg_kind = BackgroundKind::from_name(background);
                signals.background.set(bg_kind);
            }

            "action:open-github-repo" => {
                window()
                    .open_with_url_and_target("https://github.com/chriso345/chalk", "_blank")
                    .ok();
            }
            "debug:perform-action" => signals.debug.update(|n| *n += 1),
            _ => {
                leptos::logging::log!("Unknown action: {action}");
            }
        }
    });

    Effect::new({
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
        <GenerationFlow
            open=generation_palette_open
            on_commit=Callback::new(move |collection: Collection| {
                signals.collection.set(collection);
            })
        />
    }
}
