use itertools::Itertools;
use js_sys::Array;
use leptos::{html::Input, logging::log, *};
use population_dynm_sim::*;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{
    window, Blob, BlobPropertyBag, HtmlButtonElement, HtmlInputElement, MessageEvent, Url, Worker,
};

const COLORS: [(u8, u8, u8); 6] = [
    (31, 119, 180),
    (255, 126, 14),
    (44, 160, 44),
    (214, 39, 39),
    (147, 103, 189),
    (140, 86, 75),
];

fn new_worker(name: &str) -> Worker {
    let origin = window().unwrap().location().origin().unwrap();

    let script = Array::new();
    script.push(
        &format!(r#"importScripts("{origin}/population-sim-view/{name}.js");wasm_bindgen("{origin}/population-sim-view/{name}_bg.wasm");"#)
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
    let species_bytes =
        reqwest::get("https://claw89.github.io/population-sim-view/data/species_params.csv")
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
fn PlotlyChart(coords: Vec<(Vec<f64>, Vec<f64>)>) -> impl IntoView {
    let mut traces = vec![] as Vec<String>;
    for (idx, (x_coords, y_coords)) in coords.clone().iter().enumerate() {
        let (r, g, b) = COLORS[idx];
        traces.push(format!(
            "{{
                'type': 'scatter',
                'mode': 'markers',
                'x': {:?},
                'y': {:?},
                'fillcolor': 'rgb({r}, {g}, {b})'
            }}",
            x_coords, y_coords
        ));
    }

    let script = format!(
        "Plotly.newPlot('plotly_chart', {{
            'data': [{}],
            'layout': {{
                'plot_bgcolor': '#dbdbdb',
                'autosize': false,
                'width': 700,
                'height': 700,
                'xaxis': {{
                    'range': [0.0, 1.0],
                    'visible': false
                }},
                'yaxis': {{
                    'range': [0.0, 1.0],
                    'visible': false
                }}
            }},
            'config': {{}}
        }});",
        traces.join(", ")
    );

    view! {
        <div id="plotly_chart" style="width=200px; height=200px"></div>
        <script type="text/javascript">
            {script}
        </script>
    }
}

fn set_distribution(checkpoint: &Checkpoint, set_coords: WriteSignal<Vec<(Vec<f64>, Vec<f64>)>>) {
    set_coords.set(checkpoint.species_individuals.clone());
}

#[component]
fn App() -> impl IntoView {
    let worker = new_worker("worker");
    let (progress, set_progress) = create_signal::<f64>(0.0);
    let (max_t, set_max_t) = create_signal::<f64>(10.0);
    let (coords, set_coords) = create_signal::<Vec<(Vec<f64>, Vec<f64>)>>(vec![]);
    let (history_signal, set_history_signal) = create_signal::<Vec<Checkpoint>>(vec![]);

    let onmessage = Closure::wrap(Box::new(move |msg: MessageEvent| {
        let response: WorkerResponse =
            serde_wasm_bindgen::from_value(msg.data()).expect("Response type messafe");
        match response.status {
            WorkerStatus::INITIALIZED => {
                log!("app: worker ready to receive requests");
            }
            WorkerStatus::PENDING => {
                set_history_signal.update(|h| h.push(response.checkpoint.clone()));
                // update progress bar
                set_progress.set(response.checkpoint.time);
                set_distribution(&response.checkpoint, set_coords);
            }
            WorkerStatus::COMPLETE => {
                log!("app: simulation completed");
                let document = web_sys::window().unwrap().document().unwrap();
                let button = document.get_element_by_id("simulate_button").unwrap();
                button
                    .dyn_ref::<HtmlButtonElement>()
                    .unwrap()
                    .set_disabled(false);
            }
        }
    }) as Box<dyn Fn(MessageEvent)>);
    worker.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();
    log!("app: worker created");

    let species_resource = create_resource(|| (), |_| async move { load_species().await });

    let worker_clone = worker.clone();
    let view_history_node_ref = create_node_ref::<Input>();

    view! {
        <form on:input=move |ev| {
            ev.prevent_default();
            set_max_t.set(event_target_value(&ev).parse::<f64>().unwrap());
        }
        >
            <div>
                <label for="max_t_selector">"Select simulation time (s)"</label>
                <input type="number" id="max_t_selector" />
            </div>
        </form>
        <form on:submit=move |ev: leptos::ev::SubmitEvent| {
            ev.prevent_default();
            match species_resource.loading().get() {
                true => log!("app: species params are still loading"),
                false => {
                    set_history_signal.set(vec![]);
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

                    log!("app: sending simulation request");
                    let message_to_worker = WorkerMessageReceived{
                        species_list,
                        max_t: max_t.get()
                    };
                    worker_clone.post_message(&serde_wasm_bindgen::to_value(&message_to_worker).unwrap()).unwrap();
            }
        }}>
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
            <div>
                <button type="submit" id="simulate_button">"Simulate"</button>
                {move || view! {<progress id="simulation_progress" max={max_t.get()} value={progress.get()} />}}
            </div>
        </form>


        // <div style="width: 500px;" >
        // {move || view! {<MyScatterChart coords={coords.get()} /> }}
        // </div>

        <form on:input=move |_| {
            let history = history_signal.get();
            let view_idx = view_history_node_ref.get().unwrap().value_as_number() as usize;
            set_distribution(&history[view_idx], set_coords);
        }
        >
        {move || view! {
            <input
                _ref=view_history_node_ref
                type="range"
                min=0
                max={
                    if history_signal.get().len() > 0 {
                        history_signal.get().len() - 1
                    }
                    else {
                        0
                    }
                }
                value=0
                style="width: 500px;"
            />}
        }
        </form>
        {move || view! {<PlotlyChart coords={coords.get()}/>}}

    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}
