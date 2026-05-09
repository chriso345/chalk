#![allow(dead_code)]

mod canvas;
mod ui;

mod app;
mod signals;

use app::App;

use leptos::prelude::*;

fn main() {
    mount_to_body(|| view! { <App /> });
}
