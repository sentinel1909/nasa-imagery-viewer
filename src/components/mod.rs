// src/components/app.rs

use yew::{function_component, html, Html, classes};

use crate::components::header::Header;
use crate::components::content::Content;
use crate::components::footer::Footer;

mod header;
mod content;
mod footer;

#[function_component]
pub fn App() -> Html {
    html! { 
        <div class={classes!("container")}>
            <Header />
            <Content />
            <Footer />
        </div>
     }
}
