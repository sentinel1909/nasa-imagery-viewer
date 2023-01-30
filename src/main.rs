// src/main.rs

use crate::components::App;

mod components;
mod domain;

fn main() {

    yew::Renderer::<App>::new().render();
}
