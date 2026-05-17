use crate::canvas::primitives::collections::Collection;
use crate::canvas::primitives::collections::grid::GridCollection;
use leptos::prelude::*;

use crate::ui::components::{config_popup::ConfigField, config_popup::GenerationConfigPanel};

#[component]
pub fn GenerationFlow(open: RwSignal<bool>, on_commit: Callback<Collection>) -> impl IntoView {
    let pending: RwSignal<Option<Collection>> = RwSignal::new(None);
    let config_open = RwSignal::new(false);
    let config_fields = RwSignal::new(Vec::<ConfigField>::new());

    let on_select = Callback::new(move |collection: Collection| {
        let fields = collection.config_fields();
        pending.set(Some(collection));
        config_fields.set(fields);
        config_open.set(true);
    });

    let on_cancel = Callback::new(move |_| {
        pending.set(None);
    });

    let on_proceed = Callback::new(move |fields: std::collections::HashMap<String, String>| {
        if let Some(collection) = pending.get() {
            if let Some(configured) = collection.from_config(&fields) {
                on_commit.run(configured);
            }
        }
        pending.set(None);
    });

    view! {
        <GenerationPalette open=open on_select=on_select />
        <GenerationConfigPanel
            open=config_open
            fields=config_fields
            on_cancel=on_cancel
            on_proceed=on_proceed
        />
    }
}

#[derive(Clone, Debug)]
pub struct GenerationPaletteItem {
    pub label: &'static str,
    pub collection: Collection,
}

fn generation_palette_items() -> Vec<GenerationPaletteItem> {
    vec![GenerationPaletteItem {
        label: "Grid",
        collection: Collection::Grid {
            grid: GridCollection {
                origin: (0.0, 0.0),
                spacing: (50.0, 50.0),
                count: (10, 10),
            },
        },
    }]
}

#[component]
pub fn GenerationPalette(open: RwSignal<bool>, on_select: Callback<Collection>) -> impl IntoView {
    let selected = RwSignal::new(0usize);

    let close = move || open.set(false);
    let commit = move |collection: &Collection| {
        close();
        on_select.run(collection.clone());
    };

    view! {
        <Show when=move || open.get()>
            <div style="position:fixed;inset:0;z-index:200;" on:pointerdown=move |_| close() />
                <div style="position:fixed;top:20%;left:50%;transform:translateX(-50%);z-index:201;width:440px;max-height:360px;display:flex;flex-direction:column;background:rgba(242,240,239,0.97);backdrop-filter:blur(12px);border:1px solid rgba(26,26,24,0.12);box-shadow:0 8px 32px rgba(26,26,24,0.14);border-radius:12px;overflow:hidden;font-family:monospace;font-size:12px;" on:pointerdown=|e| e.stop_propagation()>
                    <div style="display:flex;gap:8px;padding:10px 14px;border-bottom:1px solid rgba(26,26,24,0.08);">
                        <span>"Select a collection type to generate"</span>
                    </div>
                    <div style="overflow-y:auto;padding:6px;">
                    {{
                        let selected = selected.clone();
                        let commit = commit.clone();
                        generation_palette_items().into_iter().enumerate().map(move |(idx, item)| {
                            let is_selected = selected.get() == idx;
                            view! {
                                <div
                                    style=move || format!(
                                        "padding:8px 12px;cursor:pointer;background:{};border-radius:8px;",
                                        if is_selected { "rgba(26,26,24,0.08)" } else { "transparent" }
                                    )
                                    on:pointerdown=move |_| commit(&item.collection)
                                >
                                    {item.label}
                                </div>
                            }
                        }).collect::<Vec<_>>()
                    }}
                </div>
            </div>
        </Show>
    }
}
