#![allow(dead_code)]

mod app;
mod canvas;
mod ui;

use app::App;
use leptos::prelude::*;

fn main() {
    mount_to_body(|| view! { <App /> });
}
