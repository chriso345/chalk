use leptos::prelude::*;
use std::collections::{HashMap, HashSet};

// Field declaration type

#[derive(Clone, Debug)]
pub enum ConfigField {
    Float {
        key: &'static str,
        label: &'static str,
        default: f64,
    },
    Int {
        key: &'static str,
        label: &'static str,
        default: i64,
    },
    Float2 {
        key_a: &'static str,
        key_b: &'static str,
        label: &'static str,
        label_a: &'static str,
        label_b: &'static str,
        default_a: f64,
        default_b: f64,
    },
    Int2 {
        key_a: &'static str,
        key_b: &'static str,
        label: &'static str,
        label_a: &'static str,
        label_b: &'static str,
        default_a: i64,
        default_b: i64,
    },
}

const BASE_INPUT_STYLE: &str = "flex:1;padding:6px;border-radius:6px;border:1px solid #ccc;\
     background:rgba(26,26,24,0.04);font-family:monospace;font-size:13px;\
     outline:none;min-width:0;box-sizing:border-box;";

const LABEL_STYLE: &str =
    "font-size:11px;color:rgba(26,26,24,0.55);margin-bottom:2px;display:block;";

const INLINE_LABEL_STYLE: &str =
    "font-size:11px;color:rgba(26,26,24,0.45);margin-right:4px;white-space:nowrap;";

#[component]
fn FloatInput(
    key: &'static str,
    label: &'static str,
    default: f64,
    values: RwSignal<HashMap<String, String>>,
    invalid_keys: RwSignal<HashSet<String>>,
) -> impl IntoView {
    let text = RwSignal::new(default.to_string());

    let on_input = move |ev: web_sys::Event| {
        let raw = event_target_value(&ev);
        text.set(raw.clone());
        if raw.trim().parse::<f64>().is_ok() {
            invalid_keys.update(|s| {
                s.remove(key);
            });
            values.update(|m| {
                m.insert(key.to_string(), raw);
            });
        } else {
            invalid_keys.update(|s| {
                s.insert(key.to_string());
            });
        }
    };

    view! {
        <div style="display:flex;flex-direction:column;gap:2px;">
            <span style=LABEL_STYLE>{label}</span>
            <input
                style=BASE_INPUT_STYLE
                style:border-color=move || if invalid_keys.get().contains(key) { "#c0392b" } else { "#ccc" }
                prop:value=text
                on:input=on_input
            />
        </div>
    }
}

#[component]
fn IntInput(
    key: &'static str,
    label: &'static str,
    default: i64,
    values: RwSignal<HashMap<String, String>>,
    invalid_keys: RwSignal<HashSet<String>>,
) -> impl IntoView {
    let text = RwSignal::new(default.to_string());

    let on_input = move |ev: web_sys::Event| {
        let raw = event_target_value(&ev);
        text.set(raw.clone());
        if raw.trim().parse::<i64>().is_ok() {
            invalid_keys.update(|s| {
                s.remove(key);
            });
            values.update(|m| {
                m.insert(key.to_string(), raw);
            });
        } else {
            invalid_keys.update(|s| {
                s.insert(key.to_string());
            });
        }
    };

    view! {
        <div style="display:flex;flex-direction:column;gap:2px;">
            <span style=LABEL_STYLE>{label}</span>
            <input
                style=BASE_INPUT_STYLE
                style:border-color=move || if invalid_keys.get().contains(key) { "#c0392b" } else { "#ccc" }
                prop:value=text
                on:input=on_input
            />
        </div>
    }
}

// Extracted from the Float2/Int2 match arms - one half of a pair row.
#[component]
fn PairHalf(
    key: &'static str,
    label: &'static str,
    default: String,
    is_float: bool,
    values: RwSignal<HashMap<String, String>>,
    invalid_keys: RwSignal<HashSet<String>>,
) -> impl IntoView {
    let text = RwSignal::new(default);

    let on_input = move |ev: web_sys::Event| {
        let raw = event_target_value(&ev);
        text.set(raw.clone());
        let ok = if is_float {
            raw.trim().parse::<f64>().is_ok()
        } else {
            raw.trim().parse::<i64>().is_ok()
        };
        if ok {
            invalid_keys.update(|s| {
                s.remove(key);
            });
            values.update(|m| {
                m.insert(key.to_string(), raw);
            });
        } else {
            invalid_keys.update(|s| {
                s.insert(key.to_string());
            });
        }
    };

    view! {
        <div style="display:flex;align-items:center;flex:1;min-width:0;">
            <span style=INLINE_LABEL_STYLE>{label}</span>
            <input
                style=BASE_INPUT_STYLE
                style:border-color=move || if invalid_keys.get().contains(key) { "#c0392b" } else { "#ccc" }
                prop:value=text
                on:input=on_input
            />
        </div>
    }
}

#[component]
fn Float2Input(
    key_a: &'static str,
    key_b: &'static str,
    label: &'static str,
    label_a: &'static str,
    label_b: &'static str,
    default_a: f64,
    default_b: f64,
    values: RwSignal<HashMap<String, String>>,
    invalid_keys: RwSignal<HashSet<String>>,
) -> impl IntoView {
    view! {
        <div style="display:flex;flex-direction:column;gap:2px;">
            <span style=LABEL_STYLE>{label}</span>
            <div style="display:flex;gap:8px;">
                <PairHalf
                    key=key_a label=label_a
                    default=default_a.to_string()
                    is_float=true
                    values=values invalid_keys=invalid_keys
                />
                <PairHalf
                    key=key_b label=label_b
                    default=default_b.to_string()
                    is_float=true
                    values=values invalid_keys=invalid_keys
                />
            </div>
        </div>
    }
}

#[component]
fn Int2Input(
    key_a: &'static str,
    key_b: &'static str,
    label: &'static str,
    label_a: &'static str,
    label_b: &'static str,
    default_a: i64,
    default_b: i64,
    values: RwSignal<HashMap<String, String>>,
    invalid_keys: RwSignal<HashSet<String>>,
) -> impl IntoView {
    view! {
        <div style="display:flex;flex-direction:column;gap:2px;">
            <span style=LABEL_STYLE>{label}</span>
            <div style="display:flex;gap:8px;">
                <PairHalf
                    key=key_a label=label_a
                    default=default_a.to_string()
                    is_float=false
                    values=values invalid_keys=invalid_keys
                />
                <PairHalf
                    key=key_b label=label_b
                    default=default_b.to_string()
                    is_float=false
                    values=values invalid_keys=invalid_keys
                />
            </div>
        </div>
    }
}

// Footer buttons extracted into components to avoid the RenderHtml tuple error
// caused by mixing static style= and reactive style:prop= on sibling elements.
#[component]
fn CancelButton(on_click: impl Fn(web_sys::MouseEvent) + 'static) -> impl IntoView {
    view! {
        <button
            on:click=on_click
            style="padding:7px 18px;border-radius:7px;border:none;background:#eee;\
                   cursor:pointer;font-family:monospace;font-size:13px;"
        >
            "Cancel"
        </button>
    }
}

#[component]
fn ProceedButton(
    on_click: impl Fn(web_sys::MouseEvent) + 'static,
    can_proceed: Signal<bool>,
) -> impl IntoView {
    view! {
        <button
            on:click=on_click
            disabled=move || !can_proceed.get()
            style="padding:7px 18px;border-radius:7px;border:none;\
                   background:#1a1a18;color:#fff;\
                   font-family:monospace;font-size:13px;transition:opacity 0.15s;"
            style:cursor=move || if can_proceed.get() { "pointer" } else { "not-allowed" }
            style:opacity=move || if can_proceed.get() { "1" } else { "0.4" }
        >
            "Proceed"
        </button>
    }
}

// Main panel

#[component]
pub fn GenerationConfigPanel(
    open: RwSignal<bool>,
    fields: RwSignal<Vec<ConfigField>>,
    on_cancel: Callback<()>,
    on_proceed: Callback<HashMap<String, String>>,
) -> impl IntoView {
    let values: RwSignal<HashMap<String, String>> = RwSignal::new(HashMap::new());
    let invalid_keys: RwSignal<HashSet<String>> = RwSignal::new(HashSet::new());

    // Reset both maps whenever the field list is replaced.
    Effect::new(move |_| {
        let fs = fields.get();
        let mut init_values = HashMap::new();
        let mut init_invalid = HashSet::new();
        for field in &fs {
            match field {
                ConfigField::Float { key, default, .. } => {
                    init_values.insert(key.to_string(), default.to_string());
                    if !default.is_finite() {
                        init_invalid.insert(key.to_string());
                    }
                }
                ConfigField::Int { key, default, .. } => {
                    init_values.insert(key.to_string(), default.to_string());
                }
                ConfigField::Float2 {
                    key_a,
                    key_b,
                    default_a,
                    default_b,
                    ..
                } => {
                    init_values.insert(key_a.to_string(), default_a.to_string());
                    init_values.insert(key_b.to_string(), default_b.to_string());
                    if !default_a.is_finite() {
                        init_invalid.insert(key_a.to_string());
                    }
                    if !default_b.is_finite() {
                        init_invalid.insert(key_b.to_string());
                    }
                }
                ConfigField::Int2 {
                    key_a,
                    key_b,
                    default_a,
                    default_b,
                    ..
                } => {
                    init_values.insert(key_a.to_string(), default_a.to_string());
                    init_values.insert(key_b.to_string(), default_b.to_string());
                }
            }
        }
        values.set(init_values);
        invalid_keys.set(init_invalid);
    });

    let can_proceed = Signal::derive(move || invalid_keys.get().is_empty());
    let close = move || open.set(false);

    let on_cancel_click = move |_: web_sys::MouseEvent| {
        close();
        on_cancel.run(());
    };

    let on_proceed_click = move |_: web_sys::MouseEvent| {
        if can_proceed.get() {
            on_proceed.run(values.get());
            close();
        }
    };

    view! {
        <Show when=move || open.get()>
            <div
                style="position:fixed;inset:0;z-index:300;"
                on:pointerdown=move |_| { close(); on_cancel.run(()); }
            />
            <div
                style="position:fixed;top:25%;left:50%;transform:translateX(-50%);z-index:301;\
                       width:400px;display:flex;flex-direction:column;\
                       background:rgba(242,240,239,0.97);backdrop-filter:blur(12px);\
                       border:1px solid rgba(26,26,24,0.12);\
                       box-shadow:0 8px 32px rgba(26,26,24,0.14);\
                       border-radius:12px;overflow:hidden;font-family:monospace;font-size:13px;"
                on:pointerdown=|e| e.stop_propagation()
            >
                // header
                <div style="padding:14px 18px;border-bottom:1px solid rgba(26,26,24,0.08);font-weight:bold;">
                    "Configure Collection"
                </div>

                // fields
                <div style="display:flex;flex-direction:column;gap:12px;padding:18px;">
                    <For
                        each=move || fields.get().into_iter().enumerate()
                        key=|(i, _)| *i
                        children=move |(_, field)| match field {
                            ConfigField::Float { key, label, default } => view! {
                                <FloatInput
                                    key=key label=label default=default
                                    values=values invalid_keys=invalid_keys
                                />
                            }.into_any(),
                            ConfigField::Int { key, label, default } => view! {
                                <IntInput
                                    key=key label=label default=default
                                    values=values invalid_keys=invalid_keys
                                />
                            }.into_any(),
                            ConfigField::Float2 { key_a, key_b, label, label_a, label_b, default_a, default_b } => view! {
                                <Float2Input
                                    key_a=key_a key_b=key_b
                                    label=label label_a=label_a label_b=label_b
                                    default_a=default_a default_b=default_b
                                    values=values invalid_keys=invalid_keys
                                />
                            }.into_any(),
                            ConfigField::Int2 { key_a, key_b, label, label_a, label_b, default_a, default_b } => view! {
                                <Int2Input
                                    key_a=key_a key_b=key_b
                                    label=label label_a=label_a label_b=label_b
                                    default_a=default_a default_b=default_b
                                    values=values invalid_keys=invalid_keys
                                />
                            }.into_any(),
                        }
                    />
                </div>

                // footer
                <div style="display:flex;gap:10px;justify-content:flex-end;padding:12px 18px;\
                            border-top:1px solid rgba(26,26,24,0.08);">
                    <CancelButton on_click=on_cancel_click />
                    <ProceedButton on_click=on_proceed_click can_proceed=can_proceed />
                </div>
            </div>
        </Show>
    }
}
