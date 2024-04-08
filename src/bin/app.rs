use itertools::Itertools;
use js_sys::Array;
use leptos::{logging::log, *};
use population_dynm_sim::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, Blob, BlobPropertyBag, MessageEvent, Url, Worker};

#[derive(Serialize, Deserialize)]
pub struct WorkerMessageReceived {
    species_list: Vec<Species>,
    max_t: f64,
}

#[derive(Serialize, Deserialize)]
pub enum WorkerStatus {
    INITIALIZED,
    COMPLETE,
}

#[derive(Serialize, Deserialize)]
pub struct WorkerResponse {
    status: WorkerStatus,
    population_size: usize,
}

fn new_worker(name: &str) -> Worker {
    let origin = window().unwrap().location().origin().unwrap();

    let script = Array::new();
    script.push(
        &format!(r#"importScripts("{origin}/{name}.js");wasm_bindgen("{origin}/{name}_bg.wasm");"#)
            .into(),
    );

    let blob = Blob::new_with_str_sequence_and_options(
        &script,
        BlobPropertyBag::new().type_("text/javascript"),
    )
    .unwrap();

    let url = Url::create_object_url_with_blob(&blob).unwrap();

    Worker::new(&url).unwrap()
}

async fn load_species() -> Vec<Species> {
    let species_bytes = reqwest::get("http://127.0.0.1:8080/data/species_params.csv")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(species_bytes.as_bytes());
    rdr.deserialize::<Species>()
        .map(|x| -> Species { x.unwrap() })
        .collect_vec()
}

#[component]
fn App() -> impl IntoView {
    let worker = new_worker("worker");
    let onmessage = Closure::wrap(Box::new(move |msg: MessageEvent| {
        let response: WorkerResponse =
            serde_wasm_bindgen::from_value(msg.data()).expect("Response type messafe");
        match response.status {
            WorkerStatus::COMPLETE => log!(
                "app: simulation completed with population size {}",
                response.population_size
            ),
            WorkerStatus::INITIALIZED => log!("app: worker ready to receive requests"),
        }
    }) as Box<dyn Fn(MessageEvent)>);
    worker.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();
    log!("app: worker created");

    let species_resource = create_resource(|| (), |_| async move { load_species().await });

    let worker_clone = worker.clone();

    view! {
        <input type="range" min=0 max=100 />
        {move || match species_resource.loading().get() {
            true => view! {<p> "loading species params" </p>},
            false => view! { <p>
                <ul>
                    {species_resource.get().unwrap().into_iter()
                        .map(|n| view! { <li>{n.id}</li>})
                        .collect::<Vec<_>>()}
                </ul></p>
            }
        }}
        <form on:submit=move |ev: leptos::ev::SubmitEvent| {
            ev.prevent_default();
            log!("app: sending simulation request");
            let message_to_worker = WorkerMessageReceived{
                species_list: species_resource.get().unwrap()[0..2].to_vec(),
                max_t: 10.0
            };
            worker_clone.post_message(&serde_wasm_bindgen::to_value(&message_to_worker).unwrap()).unwrap();
        }>
            <button type="submit">"Simulate"</button>
        </form>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}
