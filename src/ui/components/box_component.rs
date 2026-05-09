use leptos::prelude::*;

/// Visual style variant for a box.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BoxVariant {
    /// Semi-transparent panel background (default for groups).
    Panel,
    /// Invisible - no background or border.
    Ghost,
    /// Solid filled (used by badges, solid buttons).
    Solid,
}

/// Base container component. All UI elements are built on top of this.
#[component]
pub fn BoxComponent(
    #[prop(default = BoxVariant::Panel)] variant: BoxVariant,
    #[prop(default = false)] row: bool,
    #[prop(default = 8)] gap: u32,
    #[prop(default = 8)] padding: u32,
    #[prop(optional)] style_extra: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let base = "
        display:flex;
        align-items:center;
        font-family:'JetBrains Mono',ui-monospace,monospace;
        font-size:12px;
        letter-spacing:0.04em;
        user-select:none;
        border-radius:10px;
    ";

    let variant_css = match variant {
        BoxVariant::Panel => {
            "
            background:rgba(242,240,239,0.85);
            backdrop-filter:blur(8px);
            border:1px solid rgba(26,26,24,0.10);
            box-shadow:0 2px 8px rgba(26,26,24,0.06);
        "
        }
        BoxVariant::Ghost => {
            "
            background:transparent;
            border:none;
        "
        }
        BoxVariant::Solid => {
            "
            background:rgba(26,26,24,0.88);
            color:#F2F0EF;
            border:none;
        "
        }
    };

    let direction = if row { "row" } else { "column" };
    let layout_css = format!(
        "flex-direction:{};gap:{}px;padding:{}px;",
        direction, gap, padding
    );

    let extra = style_extra.unwrap_or("");
    let style = format!("{base}{variant_css}{layout_css}{extra}");

    view! {
        <div style=style>
            {children()}
        </div>
    }
}
