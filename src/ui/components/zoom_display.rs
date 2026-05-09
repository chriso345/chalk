use leptos::prelude::*;

/// Displays the current zoom percentage as a read-only badge.
#[component]
pub fn ZoomDisplay(zoom_pct: ReadSignal<u32>) -> impl IntoView {
    view! {
        <span style="
            font-family:'JetBrains Mono',ui-monospace,monospace;
            font-size:12px;
            letter-spacing:0.04em;
            color:#1a1a1888;
            text-align:right;
            pointer-events:none;
        ">
            {move || format!("{}%", zoom_pct.get())}
        </span>
    }
}
