// src/main.rs

use crate::components::App;

mod components;

fn main() {

    yew::Renderer::<App>::new().render();
}
