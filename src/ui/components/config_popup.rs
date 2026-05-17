use leptos::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct ConfigField {
    pub name: &'static str,
    pub label: &'static str,
    pub value: String,
}

#[component]
pub fn GenerationConfigPanel(
    open: RwSignal<bool>,
    fields: RwSignal<Vec<ConfigField>>,
    on_cancel: Callback<()>,
    on_proceed: Callback<HashMap<String, String>>,
) -> impl IntoView {
    let close = move || open.set(false);

    let on_cancel_click = move |_| {
        close();
        on_cancel.run(());
    };
    let on_proceed_click = move |_| {
        close();
        let map = fields
            .get()
            .iter()
            .map(|f| (f.name.to_string(), f.value.clone()))
            .collect();
        on_proceed.run(map);
    };

    view! {
        <Show when=move || open.get()>
            <div style="position:fixed;inset:0;z-index:300;" on:pointerdown=move |_| close() />
            <div style="position:fixed;top:25%;left:50%;transform:translateX(-50%);z-index:301;width:400px;display:flex;flex-direction:column;background:rgba(242,240,239,0.97);backdrop-filter:blur(12px);border:1px solid rgba(26,26,24,0.12);box-shadow:0 8px 32px rgba(26,26,24,0.14);border-radius:12px;overflow:hidden;font-family:monospace;font-size:13px;" on:pointerdown=|e| e.stop_propagation()>
                <div style="padding:14px 18px;border-bottom:1px solid rgba(26,26,24,0.08);font-weight:bold;">Configure Collection</div>
                <div style="display:flex;flex-direction:column;gap:12px;padding:18px;">
                    {fields.get().iter().enumerate().map(|(idx, field)| {
                        view! {
                            <div style="display:flex;flex-direction:column;gap:2px;">
                                <label>{field.label}</label>
                                <input type="text" value=field.value.clone() on:input=move |e| {
                                    let val = event_target_value(&e);
                                    fields.update(|fs| fs[idx].value = val);
                                } style="padding:6px;border-radius:6px;border:1px solid #ccc;" />
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
                <div style="display:flex;gap:10px;justify-content:flex-end;padding:12px 18px;border-top:1px solid rgba(26,26,24,0.08);">
                    <button on:click=on_cancel_click style="padding:7px 18px;border-radius:7px;border:none;background:#eee;">Cancel</button>
                    <button on:click=on_proceed_click style="padding:7px 18px;border-radius:7px;border:none;background:#1a1a18;color:#fff;">Proceed</button>
                </div>
            </div>
        </Show>
    }
}
