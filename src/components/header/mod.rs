// header.rs

use yew::{function_component, Html, html};

#[function_component]
pub fn Header() -> Html {
    html! {
        <header>
            <h1> { "NASA Imagery Viewer" }</h1>
            <h2> { "...a new photo or video every day"}</h2>
        </header>
    }
}