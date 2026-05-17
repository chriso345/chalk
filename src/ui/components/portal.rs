use web_sys::{Node, window};

/// Mounts a node to a fixed overlay root at the end of <body>.
/// If the overlay root does not exist, it is created.
pub fn mount_to_overlay_root(node: &Node) {
    let document = window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    let overlay_id = "chalk-overlay-root";
    let overlay_root = match document.get_element_by_id(overlay_id) {
        Some(el) => el,
        None => {
            let el = document.create_element("div").unwrap();
            el.set_id(overlay_id);
            el.set_attribute(
                "style",
                "position:fixed;inset:0;z-index:9999;pointer-events:none;",
            )
            .unwrap();
            body.append_child(&el).unwrap();
            el
        }
    };
    overlay_root.append_child(node).unwrap();
}
