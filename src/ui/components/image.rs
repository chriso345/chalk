use leptos::prelude::*;

#[component]
pub fn Image(
    src: &'static str,
    #[prop(default = 16)] size: u32,
    #[prop(optional)] alt: Option<&'static str>,
) -> impl IntoView {
    let style = format!(
        "width:{size}px;height:{size}px;object-fit:contain;display:block;pointer-events:none;"
    );
    view! {
        <img
            src=src
            alt=alt.unwrap_or("")
            style=style
        />
    }
}
