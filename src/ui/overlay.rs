use leptos::prelude::*;

use crate::signals::ChalkSignals;
use crate::ui::components::button::Button;
use crate::ui::components::hint::Hint;
use crate::ui::components::image::Image;
use crate::ui::components::zoom_display::ZoomDisplay;
use crate::ui::layout::{BoxConfig, BoxKind, Label, PanelConfig};

/// Props passed into the overlay alongside the layout config.
#[derive(Clone)]
pub struct OverlayContext {
    /// Signals for changes.
    pub signals: ChalkSignals,
    /// All button actions are forwarded here.
    pub on_action: Callback<&'static str>,
}

/// Renders the full HUD overlay from a list of `PanelConfig`s.
#[component]
pub fn Overlay(panels: Vec<PanelConfig>, ctx: OverlayContext) -> impl IntoView {
    let ctx = StoredValue::new(ctx);

    let rendered: Vec<_> = panels
        .into_iter()
        .map(|panel| render_panel(panel, ctx))
        .collect();

    view! {
        <div style="position:fixed;inset:0;pointer-events:none;z-index:100;">
            {rendered}
        </div>
    }
}

fn render_panel(panel: PanelConfig, ctx: StoredValue<OverlayContext>) -> impl IntoView {
    let anchor_css = panel.anchor.to_css(panel.offset);
    let flex_dir = panel.direction.to_flex_css();
    let gap = panel.gap;
    let padding = panel.padding;

    let position_style = format!(
        "position:fixed;{anchor_css}pointer-events:auto;\
         display:flex;flex-direction:{flex_dir};align-items:center;\
         gap:{gap}px;\
         background:rgba(242,240,239,0.85);\
         backdrop-filter:blur(8px);\
         border:1px solid rgba(26,26,24,0.10);\
         box-shadow:0 2px 8px rgba(26,26,24,0.06);\
         border-radius:10px;\
         padding:{padding}px;"
    );

    let children: Vec<_> = panel
        .children
        .into_iter()
        .map(|child| render_box(child, ctx))
        .collect();

    view! {
        <div style=position_style>
            {children}
        </div>
    }
}

use crate::canvas::primitives::ShapeKind;
use crate::canvas::tool::Tool;

fn render_box(config: BoxConfig, ctx: StoredValue<OverlayContext>) -> impl IntoView {
    let label = config.label.unwrap_or(Label::Static(""));
    let kind = config.kind;

    // Determine if this is a tool button and if it is selected
    let is_tool_button = config.id.starts_with("tool-");
    let tool_signal = ctx.with_value(|c| c.signals.tool.read_only());
    let active = Signal::derive(move || {
        if is_tool_button {
            let tool = tool_signal.get();
            match config.id {
                "tool-pan" => tool == Tool::Pan,
                "tool-pointer" => tool == Tool::Pointer,
                "tool-pen" => tool == Tool::Pen,
                "tool-line" => tool == Tool::Shape(ShapeKind::Line),
                "tool-arrow" => tool == Tool::Shape(ShapeKind::Arrow),
                "tool-rect" => tool == Tool::Shape(ShapeKind::Rect),
                "tool-circle" => tool == Tool::Shape(ShapeKind::Circle),
                _ => false,
            }
        } else {
            false
        }
    });

    match kind {
        BoxKind::Button { action } => {
            let on_click = Callback::new(move |_: ()| {
                ctx.with_value(|c| c.on_action.run(action));
            });

            // icon_button: has image children instead of a text label
            if let Some(inner_children) = config.children {
                let icons: Vec<_> = inner_children
                    .into_iter()
                    .map(|c| render_box(c, ctx))
                    .collect();
                view! {
                    <Button on_click=on_click active=active>
                        {icons}
                    </Button>
                }
                .into_any()
            } else {
                let label_signal: Signal<String> = match label {
                    Label::Static(s) => Signal::from(s.to_string()),
                    Label::Dynamic(f) => {
                        let ctx_val = ctx.get_value();
                        Signal::derive(move || f(&ctx_val))
                    }
                };

                view! {
                    <Button label=label_signal on_click=on_click active=active />
                }
                .into_any()
            }
        }

        BoxKind::Image { src } => view! { <Image src=src /> }.into_any(),

        BoxKind::Badge => {
            let zoom_pct = ctx.with_value(|c| c.signals.zoom.read_only());
            view! { <ZoomDisplay zoom_pct=zoom_pct /> }.into_any()
        }

        BoxKind::Label => {
            let txt = match label {
                Label::Static(s) => s,
                Label::Dynamic(_) => "", // TODO: Add support for dynamic labels if needed
            };

            view! {
                <Hint text=txt />
            }
            .into_any()
        }

        BoxKind::Divider => view! {
            <div style="width:1px;height:16px;background:rgba(26,26,24,0.12);margin:0 2px;" />
        }
        .into_any(),
    }
}
