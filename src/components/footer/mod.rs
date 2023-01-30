// footer.rs

use yew::{function_component, Html, html};
use chrono::{Datelike, Local};

#[function_component]
pub fn Footer() -> Html {
    let year = Local::now().year();
    html! {
        <footer>
            <section>
                <br />
                <p>{ "\u{00A9} " } {year} { " Jeffery D Mitchell | All Rights Reserved | Site created in WebAssembly with Yew for Rust" }</p>
                <p> {" Site hosting courtesy of: " }<a href="https://shuttle.rs">{ "shuttle.rs" }</a></p>
            </section>
        </footer>
    }
}