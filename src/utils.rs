use web_sys::window;

pub fn on_mount() {
    init_favicon();
}

fn init_favicon() {
    if prefers_dark_mode() {
        set_favicon("public/icons/chalkboard-dark.svg");
    } else {
        set_favicon("public/icons/chalkboard-light.svg");
    }
}

fn set_favicon(url: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    let head = document.head().unwrap();

    let existing = document.query_selector_all("link[rel~='icon']").unwrap();

    for i in 0..existing.length() {
        if let Some(node) = existing.item(i) {
            let _ = head.remove_child(&node);
        }
    }

    let link = document.create_element("link").unwrap();
    link.set_attribute("rel", "icon").unwrap();
    link.set_attribute("href", url).unwrap();
    head.append_child(&link).unwrap();
}

fn prefers_dark_mode() -> bool {
    window()
        .unwrap()
        .match_media("(prefers-color-scheme: dark)")
        .unwrap()
        .unwrap()
        .matches()
}
