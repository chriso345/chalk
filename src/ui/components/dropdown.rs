use leptos::prelude::*;

use crate::ui::{
    components::button::Button,
    components::portal::mount_to_overlay_root,
    layout::{Activation, BoxConfig, BoxKind, Direction},
    overlay::{OverlayContext, render_box},
};

#[component]
pub fn Dropdown(
    activation: Activation,
    src: String,
    items: Vec<BoxConfig>,
    ctx: StoredValue<OverlayContext>,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let hovered = RwSignal::new(false);
    let trigger_ref = NodeRef::<leptos::html::Button>::new();
    let on_click = move |_| {
        if activation == Activation::Click {
            open.update(|v| *v = !*v);
        }
    };

    let on_enter = move |_| {
        if activation == Activation::Hover {
            open.set(true);
        }
        hovered.set(true);
    };

    let on_leave = move |_| {
        if activation == Activation::Hover {
            open.set(false);
        }
        hovered.set(false);
    };
    let dropdown_style = move || {
        if !open.get() {
            return "display:none;".to_string();
        }

        let (top, left) = trigger_ref
            .get()
            .map(|el| {
                let rect = el.get_bounding_client_rect();
                (rect.bottom() + 8.0, rect.left() - 7.)
            })
            .unwrap_or((0.0, 0.0));

        format!(
            "position:fixed;top:{top}px;left:{left}px;\
        display:flex;flex-direction:column;\
        gap:2px;\
        background:rgba(242,240,239,0.95);\
        backdrop-filter:blur(8px);\
        border:1px solid rgba(26,26,24,0.10);\
        border-radius:10px;\
        padding:6px;\
        z-index:150;\
        pointer-events:auto;"
        )
    };
    let rendered_items = items
        .into_iter()
        .map(|item| {
            let item_ctx = ctx;

            if let BoxKind::Button { action } = item.kind {
                let on_click = Callback::new(move |_: ()| {
                    open.set(false);
                    item_ctx.with_value(|c| c.on_action.run(action));
                });

                let icons: Vec<_> = item
                    .children
                    .unwrap_or_default()
                    .into_iter()
                    .map(|c| render_box(c, ctx, Direction::Column))
                    .collect();

                view! {
                    <Button on_click=on_click active=Signal::from(false) hint="".to_string()>
                        {icons}
                    </Button>
                }
                .into_any()
            } else {
                render_box(item, ctx, Direction::Column).into_any()
            }
        })
        .collect::<Vec<_>>();

    let dropdown_node = NodeRef::<leptos::html::Div>::new();

    // Mount dropdown to overlay root when open
    Effect::new(move |_| {
        if open.get()
            && let Some(node) = dropdown_node.get() {
                mount_to_overlay_root(&node);
            }
    });

    view! {
        <Show when=move || open.get()>
            <div
                style="position:fixed;inset:0;z-index:149;pointer-events:auto;"
                on:pointerdown=move |_| open.set(false)
            />
        </Show>

        <button
            node_ref=trigger_ref
            style=move || {
                let is_active = open.get();
                let is_hovered = hovered.get();

                format!(
                    "cursor:pointer;padding:5px 10px;border-radius:7px;border:none;\
                    background:{};display:flex;align-items:center;gap:6px;\
                    transition:background 0.12s ease;",
                    if is_active {
                        "rgba(26,26,24,0.10)"
                    } else if is_hovered {
                        "rgba(26,26,24,0.06)"
                    } else {
                        "transparent"
                    }
                )
            }
            on:click=on_click
            on:mouseenter=on_enter
            on:mouseleave=on_leave
        >
            <img src=src style="width:16px;height:16px;" />
        </button>

        <div node_ref=dropdown_node style=dropdown_style>
            {rendered_items}
        </div>
    }
}
