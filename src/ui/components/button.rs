use leptos::prelude::*;

/// Visual style variant for a button.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonVariant {
    Ghost,
    Solid,
    Outlined,
}

#[component]
pub fn Button(
    #[prop(default = "")] label: &'static str,
    #[prop(default = ButtonVariant::Ghost)] variant: ButtonVariant,
    #[prop(default = false)] active: bool,
    on_click: Callback<()>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let (hovered, set_hovered) = signal(false);

    let button_style = move || {
        let mut s = String::from(
            "cursor:pointer;\
             padding:5px 10px;\
             border-radius:7px;\
             border:none;\
             font-family:inherit;\
             font-size:inherit;\
             letter-spacing:inherit;\
             background:transparent;\
             display:flex;align-items:center;gap:6px;\
             transition:background 0.12s ease, color 0.12s ease;\
             color:#1a1a18cc;",
        );

        if matches!(variant, ButtonVariant::Outlined) {
            s.push_str("border:1px solid rgba(26,26,24,0.20);");
        }
        if matches!(variant, ButtonVariant::Solid) {
            s.push_str("background:rgba(26,26,24,0.88);color:#F2F0EF;");
        } else if active {
            s.push_str("background:rgba(26,26,24,0.10);color:#1a1a18;font-weight:600;");
        } else if hovered.get() {
            s.push_str("background:rgba(26,26,24,0.06);color:#1a1a18;");
        }

        s
    };

    view! {
        <button
            style=button_style
            on:click=move |_| on_click.run(())
            on:mouseenter=move |_| set_hovered.set(true)
            on:mouseleave=move |_| set_hovered.set(false)
        >
            // Render icon children if present, otherwise the text label
            {match children {
                Some(c) => view! { <>{c()}</> }.into_any(),
                None    => view! { <>{label}</> }.into_any(),
            }}
        </button>
    }
}
