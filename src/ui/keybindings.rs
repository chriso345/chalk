use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::KeyboardEvent;

/// A single keybinding rule.
pub struct Keybinding {
    /// The `KeyboardEvent.key` value to match (case-sensitive).
    pub key: &'static str,
    pub ctrl: bool,
    pub shift: bool,
    /// The action string forwarded to `on_action`.
    pub action: &'static str,
}

impl Keybinding {
    const fn simple(key: &'static str, action: &'static str) -> Self {
        Self {
            key,
            ctrl: false,
            shift: false,
            action,
        }
    }

    const fn ctrl(key: &'static str, action: &'static str) -> Self {
        Self {
            key,
            ctrl: true,
            shift: false,
            action,
        }
    }

    const fn ctrl_shift(key: &'static str, action: &'static str) -> Self {
        Self {
            key,
            ctrl: true,
            shift: true,
            action,
        }
    }

    fn matches(&self, e: &KeyboardEvent) -> bool {
        e.key() == self.key && e.ctrl_key() == self.ctrl && e.shift_key() == self.shift
    }
}

/// All global keybindings. Evaluated top-to-bottom; first match wins.
static KEYBINDINGS: &[Keybinding] = &[
    // Clear cursor
    Keybinding::simple("Escape", "tool:pointer"),
    // Palette
    Keybinding::simple(" ", "ui:open-palette"),
    // Tools
    Keybinding::simple("q", "action:lock-tool"),
    Keybinding::simple("v", "tool:pan"),
    Keybinding::simple("p", "tool:pen"),
    Keybinding::simple("l", "tool:line"),
    Keybinding::simple("a", "tool:arrow"),
    Keybinding::simple("r", "tool:rect"),
    Keybinding::simple("c", "tool:circle"),
    // History
    Keybinding::ctrl("z", "action:undo"),
    Keybinding::ctrl_shift("z", "action:redo"),
    Keybinding::ctrl("y", "action:redo"),
    // Zoom
    Keybinding::ctrl("=", "action:zoom-in"),
    Keybinding::ctrl("-", "action:zoom-out"),
    Keybinding::ctrl("0", "action:reset-zoom"),
];

/// Register a global `keydown` listener that fires `on_action` for any
/// matching keybinding. Call this once from `App` on mount.
///
/// The returned `Closure` must be kept alive for as long as the listener
/// should be active. Typically you call `.forget()` to leak it for the
/// lifetime of the page.
pub fn register(on_action: Callback<&'static str>) {
    let closure = Closure::<dyn Fn(KeyboardEvent)>::new(move |e: KeyboardEvent| {
        // Don't intercept keys while the user is typing in an input/textarea.
        if is_typing_target(&e) {
            return;
        }

        for binding in KEYBINDINGS {
            if binding.matches(&e) {
                e.prevent_default();
                on_action.run(binding.action);
                return;
            }
        }
    });

    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
        .unwrap();

    closure.forget();
}

/// Returns true if the event target is an editable element where we should
/// not intercept keypresses.
fn is_typing_target(e: &KeyboardEvent) -> bool {
    use wasm_bindgen::JsCast;

    let Some(target) = e.target() else {
        return false;
    };

    if let Some(el) = target.dyn_ref::<web_sys::HtmlInputElement>() {
        return !el.read_only();
    }
    if let Some(el) = target.dyn_ref::<web_sys::HtmlTextAreaElement>() {
        return !el.read_only();
    }
    false
}
