// src/components/app.rs

use yew::{function_component, html, Html, use_state, use_effect_with_deps};
use chrono::{Datelike, Local};
use serde::Deserialize;
use gloo_net::http::Request;

#[derive(Deserialize, Debug, Clone)]
struct NASAData {
    date: String,
    title: String,
    explanation: String,
    url: String,
}

#[function_component]
fn Header() -> Html {
    html! {
        <header>
            <h1> { "NASA Imagery Viewer" }</h1>
            <h2> { "...a new photo or video every day"}</h2>
        </header>
    }
}

#[function_component]
fn Content() -> Html {
    let fetched_data = use_state(|| NASAData{ date: "".to_string(), title: "".to_string(), explanation: "".to_string(), url: "".to_string() });
    {
        let fetched_data = fetched_data.clone();
        use_effect_with_deps(move |_| {
               let fetched_data = fetched_data.clone();
               wasm_bindgen_futures::spawn_local(async move {
                    let fetched_nasa_data: NASAData = Request::get("https://api.nasa.gov/planetary/apod?api_key=lsULnkmChaJlS3fZO85M3cnGA8TFCAm2peEfd9QS")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                   fetched_data.set(fetched_nasa_data);
                });
                || ()
        }, ());
    }
    html! {
        <main>
            <section>
                <h3>{ "Date: " } {&fetched_data.date}</h3>
                <h3>{ "Title: " } {&fetched_data.title}</h3>
                <h3>{ "Explanation: " } </h3>
                <p> {&fetched_data.explanation} </p>
                <h3> { "Image: " } </h3>
                <img src={fetched_data.url.clone()} alt={"NASA Astronomy Photo of the Day "} />
            </section>
        </main>
    }
}

#[function_component]
fn Footer() -> Html {
    let year = Local::now().year();
    html! {
        <footer>
            <section>
                <p>{ "\u{00A9} " } {year} { " Jeffery D Mitchell | All Rights Reserved | Site created in WebAssembly with Yew for Rust" }</p>
            </section>
        </footer>
    }
}

#[function_component]
pub fn App() -> Html {
    html! { 
        <div>
            <Header />
            <Content />
            <Footer />
        </div>
     }
}
