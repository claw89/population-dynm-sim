use itertools::Itertools;
use js_sys::Array;
use leptos::{html::Input, logging::log, *};
use leptos_chart::*;
use population_dynm_sim::*;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{
    window, Blob, BlobPropertyBag, HtmlButtonElement, HtmlInputElement, MessageEvent, Url, Worker,
};

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
fn MyScatterChart(coords: Vec<(f64, f64)>) -> impl IntoView {
    let (x_coords, y_coords): (Vec<f64>, Vec<f64>) = coords.iter().cloned().unzip();
    let chart = Cartesian::new(
        Series::from(x_coords).set_range(0.0, 1.0),
        Series::from(y_coords).set_range(0.0, 1.0),
    )
    .set_view(620, 620, 3, 100, 100, 20);

    view! {
        // color is option
        <ScatterChart chart=chart />
    }
}

fn set_distribution(checkpoint: &Checkpoint, set_coords: WriteSignal<Vec<(f64, f64)>>) {
    let new_coords = checkpoint
        .x_coords
        .iter()
        .zip(checkpoint.y_coords.iter())
        .map(|(x, y)| (*x, *y))
        .collect();
    set_coords.set(new_coords);
}

#[component]
fn App() -> impl IntoView {
    let worker = new_worker("worker");
    let (coords, set_coords) = create_signal::<Vec<(f64, f64)>>(vec![]);
    let (population_signal, set_population_signal) = create_signal(Population::new(vec![]));

    let onmessage = Closure::wrap(Box::new(move |msg: MessageEvent| {
        let response: WorkerResponse =
            serde_wasm_bindgen::from_value(msg.data()).expect("Response type messafe");
        match response.status {
            WorkerStatus::COMPLETE => {
                log!(
                    "app: simulation completed with population size {} in {} steps",
                    response.population.size,
                    response.population.history.checkpoints.len()
                );
                set_population_signal.set(response.population);

                let document = web_sys::window().unwrap().document().unwrap();
                let button = document.get_element_by_id("simulate_button").unwrap();
                button
                    .dyn_ref::<HtmlButtonElement>()
                    .unwrap()
                    .set_disabled(false);
            }
            WorkerStatus::INITIALIZED => log!("app: worker ready to receive requests"),
        }
    }) as Box<dyn Fn(MessageEvent)>);
    worker.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();
    log!("app: worker created");

    let species_resource = create_resource(|| (), |_| async move { load_species().await });

    let worker_clone = worker.clone();
    let max_t_node_ref = create_node_ref::<Input>();
    let view_history_node_ref = create_node_ref::<Input>();

    view! {

        <form on:submit=move |ev: leptos::ev::SubmitEvent| {
            ev.prevent_default();
            match species_resource.loading().get() {
                true => log!("app: species params are still loading"),
                false => {
                    let document = web_sys::window().unwrap().document().unwrap();
                    let button = document.get_element_by_id("simulate_button").unwrap();
                    button.dyn_ref::<HtmlButtonElement>().unwrap().set_disabled(true);

                    let all_species = species_resource.get().unwrap();
                    let checked_species = (0..6).map(|id| {
                        document
                            .get_element_by_id(&format!("species_{}", id))
                            .unwrap()
                            .dyn_ref::<HtmlInputElement>()
                            .unwrap()
                            .checked()
                    });
                    let species_list = checked_species
                        .enumerate()
                        .filter(|(_, check)| *check)
                        .map(|(index, _)| all_species[index])
                        .collect::<Vec<Species>>();
                    let max_t = max_t_node_ref.get().unwrap().value_as_number();

                    log!("app: sending simulation request");
                    let message_to_worker = WorkerMessageReceived{
                        species_list,
                        max_t
                    };
                    worker_clone.post_message(&serde_wasm_bindgen::to_value(&message_to_worker).unwrap()).unwrap();
            }
        }}>
        <div>
            <label for="max_t_selector">"Select simulation time (s)"</label>
            <input _ref=max_t_node_ref type="number" id="max_t_selector" />
        </div>
        {move || {
            match species_resource.loading().get() {
                true => view! {<p> "loading species params" </p>},
                false => view! { <p>
                        {species_resource.get().unwrap().into_iter()
                            .map(|n| {
                                view! {
                                    <div>
                                    <input
                                        type="checkbox"
                                        id={format!{"species_{}", n.id}}
                                        name={format!{"species_{}", n.id}}
                                    />
                                    <label for=format!{"species_{}", n.id}>{format!{"Species {}", n.id}}</label>
                                    </div>
                        }})
                        .collect::<Vec<_>>()}
                    </p>
                }
            }
        }}
            <button type="submit" id="simulate_button">"Simulate"</button>
        </form>


        <div style="width: 500px;" >
        {move || view! {<MyScatterChart coords={coords.get()} /> }}
        </div>

        <form on:input=move |_| {
            let current_population = population_signal.get();
            let view_idx = view_history_node_ref.get().unwrap().value_as_number() as usize;
            set_distribution(&current_population.history.checkpoints[view_idx], set_coords);
        }
        >
        {move || view! {
            <input
                _ref=view_history_node_ref
                type="range"
                min=0
                max={population_signal.get().history.checkpoints.len() - 1}
                value=0
                style="width: 500px;"
            />}
        }
        </form>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}
