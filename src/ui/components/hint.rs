use leptos::prelude::*;

/// A static hint label shown at the bottom of the screen.
#[component]
pub fn Hint(text: &'static str) -> impl IntoView {
    view! {
        <span style="
            font-family:'JetBrains Mono',ui-monospace,monospace;
            font-size:11px;
            letter-spacing:0.06em;
            color:#1a1a1855;
            white-space:nowrap;
            pointer-events:none;
        ">
            {text}
        </span>
    }
}
