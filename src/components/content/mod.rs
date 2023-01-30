// content.rs

use yew::{function_component, Html, html, classes, use_state, use_effect_with_deps};
use gloo_net::http::Request;
use url::Url;

use crate::domain::NASAData;

#[function_component]
pub fn Content() -> Html {
    let nasa_api_key = "lsULnkmChaJlS3fZO85M3cnGA8TFCAm2peEfd9QS";
    let api_key = ["apod?api_key=", &nasa_api_key].concat();
    let api_url = Url::parse("https://api.nasa.gov/planetary/").unwrap();
    let api_url = api_url.join(&api_key).expect("Failed to join URL");
    let fetched_data = use_state(|| NASAData{ date: "".to_string(), title: "".to_string(), explanation: "".to_string(), hdurl: "".to_string() });
    {
        let fetched_data = fetched_data.clone();
        use_effect_with_deps(move |_| {
               let fetched_data = fetched_data.clone();
               wasm_bindgen_futures::spawn_local(async move {
                    let fetched_nasa_data: NASAData = Request::get(&api_url.to_string())
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
                <img src={fetched_data.hdurl.clone()} class={classes!("img-fluid")} alt={"NASA Astronomy Photo of the Day "} />
            </section>
        </main>
    }
}