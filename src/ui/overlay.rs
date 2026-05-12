use leptos::prelude::*;

use crate::signals::ChalkSignals;
use crate::ui::components::button::Button;
use crate::ui::components::hint::Hint;
use crate::ui::components::image::Image;
use crate::ui::layout::{BoxConfig, BoxKind, Direction, Label, PanelConfig};

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
        .map(|child| render_box(child, ctx, panel.direction))
        .collect();

    view! {
        <div style=position_style>
            {children}
        </div>
    }
}

use crate::canvas::primitives::ShapeKind;
use crate::canvas::tool::Tool;

fn render_box(
    config: BoxConfig,
    ctx: StoredValue<OverlayContext>,
    direction: Direction,
) -> impl IntoView {
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
                "tool-circle" => tool == Tool::Shape(ShapeKind::Oval),
                "tool-lock" => ctx.with_value(|c| c.signals.lock_tool.get()),
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
                let hint = match config.hint {
                    Some(s) => s,
                    None => "",
                };

                let icons: Vec<_> = inner_children
                    .into_iter()
                    .map(|c| render_box(c, ctx, direction))
                    .collect();
                view! {
                    <Button on_click=on_click active=active hint=hint.to_string()>
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

        BoxKind::Divider => {
            let style = match direction {
                Direction::Row => {
                    "width:1px;height:16px;background:rgba(26,26,24,0.12);margin:0 2px;"
                }
                Direction::Column => {
                    "height:1px;width:16px;background:rgba(26,26,24,0.12);margin:2px 0;"
                }
            };
            view! { <div style=style /> }.into_any()
        }

        BoxKind::Swatch { color, action } => {
            let on_click = Callback::new(move |_: ()| {
                ctx.with_value(|c| c.on_action.run(action));
            });
            let (hovered, set_hovered) = signal(false);
            let style = move || {
                format!(
                    "width:22px;height:22px;border-radius:50%;background:{color};\
         border:{};\
         box-shadow:{};\
         cursor:pointer;transition:transform 0.1s;\
         transform:{};",
                    if active.get() {
                        "2.5px solid rgba(26,26,24,0.8)"
                    } else {
                        "1.5px solid rgba(26,26,24,0.15)"
                    },
                    if active.get() {
                        "inset 0 0 0 2px #F2F0EF"
                    } else {
                        "none"
                    },
                    if hovered.get() && !active.get() {
                        "scale(1.12)"
                    } else {
                        "scale(1)"
                    },
                )
            };
            view! {
                <button style=style
                    on:click=move |_| on_click.run(())
                    on:mouseenter=move |_| set_hovered.set(true)
                    on:mouseleave=move |_| set_hovered.set(false)
                />
            }
            .into_any()
        }

        BoxKind::StrokeWidth { width, action } => {
            let on_click = Callback::new(move |_: ()| {
                ctx.with_value(|c| c.on_action.run(action));
            });
            let (hovered, set_hovered) = signal(false);
            let btn_style = move || {
                format!(
                    "width:32px;height:32px;border-radius:7px;border:none;\
         background:{};cursor:pointer;\
         display:flex;align-items:center;justify-content:center;\
         transition:background 0.1s;",
                    if active.get() {
                        "rgba(26,26,24,0.10)"
                    } else if hovered.get() {
                        "rgba(26,26,24,0.05)"
                    } else {
                        "transparent"
                    }
                )
            };
            // Clamp visual thickness so thick strokes don't overflow the button
            let visual_w = (width as f64).min(5.0);
            view! {
                <button style=btn_style
                    on:click=move |_| on_click.run(())
                    on:mouseenter=move |_| set_hovered.set(true)
                    on:mouseleave=move |_| set_hovered.set(false)
                >
                    <svg width="20" height="20" viewBox="0 0 20 20"
                         style="pointer-events:none;display:block;">
                        <line x1="3" y1="10" x2="17" y2="10"
                              stroke="#1A1A18"
                              stroke-width=visual_w.to_string()
                              stroke-linecap="round"/>
                    </svg>
                </button>
            }
            .into_any()
        }
    }
}
