use leptos::prelude::*;
use web_sys::KeyboardEvent;

/// A single item shown in the palette.
#[derive(Clone, Debug)]
pub struct PaletteItem {
    /// Displayed name.
    pub label: &'static str,
    /// Optional category shown in muted text beside the label.
    pub category: &'static str,
    /// Action string forwarded to `on_action` when selected.
    pub action: &'static str,
    /// Optional keyboard shortcut hint.
    pub shortcut: Option<&'static str>,
}

impl PaletteItem {
    const fn new(
        label: &'static str,
        category: &'static str,
        action: &'static str,
        shortcut: Option<&'static str>,
    ) -> Self {
        Self {
            label,
            category,
            action,
            shortcut,
        }
    }
}

/// All items available in the palette.
static PALETTE_ITEMS: &[PaletteItem] = &[
    // Tools
    PaletteItem::new("Pan", "Tool", "tool:pan", Some("v")),
    PaletteItem::new("Pen", "Tool", "tool:pen", Some("p")),
    PaletteItem::new("Line", "Tool", "tool:line", Some("l")),
    PaletteItem::new("Arrow", "Tool", "tool:arrow", Some("a")),
    PaletteItem::new("Rect", "Tool", "tool:rect", Some("r")),
    PaletteItem::new("Circle", "Tool", "tool:circle", Some("c")),
    // Actions
    PaletteItem::new("Undo", "Action", "action:undo", Some("Ctrl+Z")),
    PaletteItem::new("Redo", "Action", "action:redo", Some("Ctrl+Shift+Z")),
    PaletteItem::new("Clear", "Action", "action:clear", None),
    PaletteItem::new("Reset Zoom", "Action", "action:reset-zoom", Some("Ctrl+0")),
    PaletteItem::new("Toggle Theme", "Action", "action:toggle-dark-mode", None),
    // Generation
    PaletteItem::new("Generate", "", "ui:open-generation-palette", None),
];

#[component]
pub fn Palette(open: RwSignal<bool>, on_action: Callback<&'static str>) -> impl IntoView {
    let query = RwSignal::new(String::new());
    let selected = RwSignal::new(0usize);

    // Filter palette items
    let filtered = move || {
        let q = query.get().to_lowercase();
        PALETTE_ITEMS
            .iter()
            .filter(|item| {
                q.is_empty()
                    || item.label.to_lowercase().contains(&q)
                    || item.category.to_lowercase().contains(&q)
            })
            .collect::<Vec<&PaletteItem>>()
    };

    // Reset when opened
    Effect::new(move |_| {
        if open.get() {
            query.set(String::new());
            selected.set(0);
        }
    });

    let clamp_selected = move || {
        let len = filtered().len();
        if len == 0 {
            selected.set(0);
        } else {
            selected.update(|i| *i = (*i).min(len - 1));
        }
    };

    let close = move || open.set(false);

    let commit = move |action: &'static str| {
        close();
        on_action.run(action);
    };

    let on_keydown = move |e: KeyboardEvent| match e.key().as_str() {
        "Escape" => {
            e.prevent_default();
            close();
        }
        "ArrowDown" => {
            e.prevent_default();
            let len = filtered().len();
            if len > 0 {
                selected.update(|i| *i = (*i + 1) % len);
            }
        }
        "ArrowUp" => {
            e.prevent_default();
            let len = filtered().len();
            if len > 0 {
                selected.update(|i| *i = if *i == 0 { len - 1 } else { *i - 1 });
            }
        }
        "Enter" => {
            e.prevent_default();
            let items = filtered();
            if let Some(item) = items.get(selected.get()) {
                commit(item.action);
            }
        }
        _ => {}
    };

    let input_ref = NodeRef::<leptos::html::Input>::new();

    Effect::new(move |_| {
        if open.get()
            && let Some(el) = input_ref.get() {
                let _ = el.focus();
            }
    });

    view! {
        <Show when=move || open.get()>
            <div
                style="position:fixed;inset:0;z-index:200;"
                on:pointerdown=move |_| close()
            />

            <div
                style="
                    position:fixed;
                    top:20%;
                    left:50%;
                    transform:translateX(-50%);
                    z-index:201;
                    width:440px;
                    max-height:360px;
                    display:flex;
                    flex-direction:column;
                    background:rgba(242,240,239,0.97);
                    backdrop-filter:blur(12px);
                    border:1px solid rgba(26,26,24,0.12);
                    box-shadow:0 8px 32px rgba(26,26,24,0.14);
                    border-radius:12px;
                    overflow:hidden;
                    font-family:monospace;
                    font-size:12px;
                "
                on:pointerdown=|e| e.stop_propagation()
            >
                <div style="display:flex;gap:8px;padding:10px 14px;border-bottom:1px solid rgba(26,26,24,0.08);">
                    <input
                        node_ref=input_ref
                        type="text"
                        placeholder="Search…"
                        style="flex:1;border:none;outline:none;background:transparent;"
                        on:input=move |e| {
                            let val = event_target_value(&e);
                            query.set(val);
                            clamp_selected();
                        }
                        on:keydown=on_keydown
                    />
                </div>

                <div style="overflow-y:auto;padding:6px;">
                    <For
                        each=move || filtered().into_iter().enumerate()
                        key=|(_, item)| item.action
                        children=move |(idx, item): (usize, &PaletteItem)| {
                            let action = item.action;
                            let is_selected = move || selected.get() == idx;

                            let row_style = move || {
                                if is_selected() {
                                    "background:rgba(26,26,24,0.08);display:flex;justify-content:space-between;padding:7px 10px;border-radius:7px;cursor:pointer;".to_string()
                                } else {
                                    "display:flex;justify-content:space-between;padding:7px 10px;border-radius:7px;cursor:pointer;".to_string()
                                }
                            };

                            view! {
                                <div
                                    style=row_style
                                    on:pointerenter=move |_| selected.set(idx)
                                    on:pointerdown=move |e| {
                                        e.prevent_default();
                                        commit(action);
                                    }
                                >
                                    <div style="display:flex;gap:8px;">
                                        <span>{item.label}</span>
                                        <span style="opacity:0.5;font-size:10px;">
                                            {item.category}
                                        </span>
                                    </div>

                                    {item.shortcut.map(|s| view! {
                                        <span style="opacity:0.5;font-size:10px;">{s}</span>
                                    })}
                                </div>
                            }
                        }
                    />

                    <Show when=move || filtered().is_empty()>
                        <div style="padding:16px;text-align:center;opacity:0.5;">
                            "No results"
                        </div>
                    </Show>
                </div>
            </div>
        </Show>
    }
}
