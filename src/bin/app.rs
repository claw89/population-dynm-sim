use itertools::Itertools;
use js_sys::Array;
use leptos::{logging::log, *};
use population_dynm_sim::*;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{
    window, Blob, BlobPropertyBag, HtmlButtonElement, HtmlInputElement, MessageEvent, Url, Worker,
};

// Tab10 RBG colors
const COLORS: [(u8, u8, u8); 6] = [
    (31, 119, 180),
    (255, 126, 14),
    (44, 160, 44),
    (214, 39, 39),
    (147, 103, 189),
    (140, 86, 75),
];

fn new_worker(name: &str) -> Worker {
    // Creates a web worker; 'bin/{name}.rs' should contain the worker's internal logic,
    // and should be referenced in index.html as:
    // <link data-trunk rel="rust" href="Cargo.toml" data-wasm-opt="z" data-bin="{name}" data-type="worker" />
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
    // Loads the species data from ./data/species_params.csv; data dir should be referenced in index.html as
    // <link data-trunk rel="copy-dir" href="data" />
    let document = web_sys::window().unwrap().document().unwrap();
    let species_bytes =
        reqwest::get(format! {"{}/data/species_params.csv", document.url().unwrap()})
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(species_bytes.as_bytes());
    rdr.deserialize::<Species>()
        .map(|x| -> Species {
            let mut species = x.unwrap();
            species.derive_norms();
            species
        })
        .collect_vec()
}

#[component]
fn PlotlyChart(div_id: String, size: (f64, f64)) -> impl IntoView {
    // A script component that generates an emply plotly chart
    let (min, max) = size;
    let script = format!(
        "Plotly.newPlot('{}', {{
            'data': [],
            'layout': {{
                'margin': {{
                    'l': 0,
                    'r': 0,
                    'b': 0,
                    't': 0,
                    'pad': 4
                }},
                'showlegend': false,
                'plot_bgcolor': '#dbdbdb',
                'autosize': false,
                'width': 400,
                'height': 400,
                'xaxis': {{
                    'range': [{}, {}],
                    'visible': false
                }},
                'yaxis': {{
                    'range': [{}, {}],
                    'visible': false
                }}
            }},
            'config': {{}}
        }});",
        div_id, min, max, min, max
    );

    view! {
        <script type="text/javascript">
            {script}
        </script>
    }
}

#[component]
fn UpdateChart(coords: Vec<SpeciesCoords>, div_id: String) -> impl IntoView {
    // A script component that updates the traces in a plotly chart
    let mut traces = vec![] as Vec<String>;
    for species_coords in coords.into_iter() {
        let (r, g, b) = COLORS[species_coords.species_id];
        traces.push(format!(
            "{{
                'type': 'scatter',
                'mode': 'markers',
                'x': {:?},
                'y': {:?},
                marker: {{
                  'color': 'rgb({r}, {g}, {b})',
                }}
            }}",
            species_coords.x_coords, species_coords.y_coords
        ));
    }

    let mut n_traces = 0;
    let document = web_sys::window().unwrap().document().unwrap();
    let scatter_layer = document.get_elements_by_class_name("scatterlayer mlayer");
    if scatter_layer.length() > 0 {
        assert_eq!(scatter_layer.length(), 1);
        n_traces = scatter_layer.item(0).unwrap().children().length();
    }

    let mut delete_traces = String::from("");
    if n_traces > 0 {
        delete_traces = (0..n_traces)
            .collect_vec()
            .iter()
            .map(|t| format!("{:?}", t))
            .collect::<Vec<String>>()
            .join(", ");
    }

    let script = format!(
        "
        Plotly.deleteTraces('{}', [{}]);
        Plotly.addTraces('{}', [{}]);
        ",
        div_id,
        delete_traces,
        div_id,
        traces.join(", ")
    );

    view! {
        <script type="text/javascript">
            {script}
        </script>
    }
}

#[component]
fn UpdateHeatmap(
    heatmap: Vec<Vec<f64>>,
    div_id: String,
    history: Vec<Checkpoint>,
) -> impl IntoView {
    let trace = format!(
        "{{
            'type': 'heatmap',
            'colorscale': [
                [0, '#dbdbdb'],
                [1, '#1F77B4']
            ],
            'showscale': false,
            'z': {:?},
        }}",
        heatmap
    );

    let mut delete_traces = String::from("");
    if !history.is_empty() {
        delete_traces = format!("Plotly.deleteTraces('{}', [0]);", div_id)
    }

    let script = format!(
        "
        {}
        Plotly.addTraces('{}', [{}]);
        ",
        delete_traces, div_id, trace
    );
    view! {
        <script type="text/javascript">
            {script}
        </script>
    }
}

fn set_distribution(checkpoint: &Checkpoint, set_coords: WriteSignal<Vec<SpeciesCoords>>) {
    set_coords.set(checkpoint.species_individuals.clone());
}

fn worker_onmessage(
    set_history: WriteSignal<Vec<Checkpoint>>,
    set_progress: WriteSignal<f64>,
    set_coords: WriteSignal<Vec<SpeciesCoords>>,
    set_heatmap: WriteSignal<Vec<Vec<f64>>>,
) -> Closure<dyn Fn(MessageEvent)> {
    // Defines the actions to take when recieving a message from the worker
    Closure::wrap(Box::new(move |msg: MessageEvent| {
        let response: WorkerResponse =
            serde_wasm_bindgen::from_value(msg.data()).expect("Response type messafe");
        match response.status {
            WorkerStatus::INITIALIZED => {
                log!("app: worker ready to receive requests");
            }
            WorkerStatus::PENDING => {
                set_history.update(|h| h.append(&mut response.checkpoints.clone()));
                set_progress.set(response.checkpoints.last().unwrap().time);
                set_distribution(response.checkpoints.last().unwrap(), set_coords);
                set_heatmap.set(response.checkpoints.last().unwrap().heatmap[0].clone())
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
    }) as Box<dyn Fn(MessageEvent)>)
}

fn run_simulation(
    ev: leptos::ev::SubmitEvent,
    species_resource: Resource<(), Vec<Species>>,
    set_history: WriteSignal<Vec<Checkpoint>>,
    checked_species: ReadSignal<Vec<usize>>,
    max_t: ReadSignal<f64>,
    worker: Worker,
) {
    // Defines the actions to take when the user initiates a simulation
    ev.prevent_default();
    match species_resource.loading().get() {
        true => log!("app: species params are still loading"),
        false => {
            set_history.set(vec![]);
            let document = web_sys::window().unwrap().document().unwrap();
            let button = document.get_element_by_id("simulate_button").unwrap();
            button
                .dyn_ref::<HtmlButtonElement>()
                .unwrap()
                .set_disabled(true);

            let all_species = species_resource.get().unwrap();
            let mut submited_species = checked_species.get().clone();
            submited_species.sort();
            let species_list = submited_species
                .iter()
                .map(|index| all_species[*index])
                .collect::<Vec<Species>>();

            log!("app: sending simulation request");
            let message_to_worker = WorkerMessageReceived {
                species_list,
                max_t: max_t.get(),
            };
            worker
                .post_message(&serde_wasm_bindgen::to_value(&message_to_worker).unwrap())
                .unwrap();
        }
    }
}

#[component]
fn SpeciesSelector(
    species_resource: Resource<(), Vec<Species>>,
    species_detail: ReadSignal<usize>,
    set_species_detail: WriteSignal<usize>,
    checked_species: ReadSignal<Vec<usize>>,
    set_checked_species: WriteSignal<Vec<usize>>,
) -> impl IntoView {
    // A component with which a user can choose a selection of different species for simulation
    move || match species_resource.loading().get() {
        true => view! { <div id="tabs"></div>},
        false => view! {  <div id="tabs">
                {species_resource.get().unwrap().into_iter()
                    .map(|n| {
                        view! {
                            <div
                                id="species_tab"
                                style={
                                    let mut style = "padding: 4px; display:flex; flex-direction: row; justify-content: left; align-items: top; ".to_string();
                                    if n.id == species_detail.get() {
                                        style.push_str("border-style: solid; border-width: 1px; border-right-style: none; ");
                                    }
                                    else {
                                        style.push_str("background: #dbdbdb; border-width: 1px; border-right-style: solid; ");
                                    }
                                    style
                                }>
                            <input
                                type="checkbox"
                                id=format!("species_{}_checkbox", n.id)
                                on:input=move |ev| {
                                    match event_target_checked(&ev) {
                                        true => set_checked_species.update(|v| v.push(n.id)),
                                        false => set_checked_species.update(|v| v.retain(|&x| x != n.id))
                                    }
                                }
                            />
                            <p on:click=move |_| {
                                set_species_detail.set(n.id);
                                let document = web_sys::window().unwrap().document().unwrap();
                                for checked_id in checked_species.get() {
                                    document.get_element_by_id(&format!("species_{}_checkbox", checked_id))
                                        .unwrap()
                                        .dyn_ref::<HtmlInputElement>()
                                        .unwrap()
                                        .set_checked(true);
                                }
                            }
                            style="margin: 0"
                            >{format!{"Species {}", n.id}}</p>
                            </div>
                }})
                .collect::<Vec<_>>()}
            </div>
        },
    }
}

#[component]
fn SpeciesDetail0(
    species_resource: Resource<(), Vec<Species>>,
    species_detail: ReadSignal<usize>,
) -> impl IntoView {
    // A component showing the first column of species details
    move || match species_resource.loading().get() {
        true => view! {<div id="details_0"></div>},
        false => view! { <div id="details_0">
                {species_resource.get().unwrap().into_iter()
                    .map(|n| {
                        view! {
                            <div id=format!{"species_{}_details_c0", n.id} style={
                                if n.id == species_detail.get() {
                                    "display: block; visibility: visible"
                                }
                                else {
                                    "display: none; visibility: hidden"
                                }
                            }>
                                <ul>
                                    <li>{format!{"b0: {}", n.b0}}</li>
                                    <li>{format!{"b1: {}", n.b1}}</li>
                                    <li>{format!{"c1: {}", n.c1}}</li>
                                    <li>{format!{"d0: {}", n.d0}}</li>
                                    <li>{format!{"d1: {}", n.d1}}</li>
                                    <li>{format!{"mbrmax: {}", n.mbrmax}}</li>
                                    <li>{format!{"mbsd: {}", n.mbsd}}</li>
                                </ul>
                            </div>
                }})
                .collect::<Vec<_>>()}
            </div>
        },
    }
}

#[component]
fn SpeciesDetail1(
    species_resource: Resource<(), Vec<Species>>,
    species_detail: ReadSignal<usize>,
) -> impl IntoView {
    // A component showing the second column of species details
    move || match species_resource.loading().get() {
        true => view! {<div id="details_1"></div>},
        false => view! { <div id="details_1">
                {species_resource.get().unwrap().into_iter()
                    .map(|n| {
                        view! {
                            <div id=format!{"species_{}_details_c1", n.id} style={
                                if n.id == species_detail.get() {
                                    "display: block; visibility: visible"
                                }
                                else {
                                    "display: none; visibility: hidden"
                                }
                            }>
                                <ul>
                                    <li>{format!{"mintegral: {}", n.mintegral}}</li>
                                    <li>{format!{"move_radius_max: {}", n.move_radius_max}}</li>
                                    <li>{format!{"move_std: {}", n.move_std}}</li>
                                    <li>{format!{"birth_radius_max: {}", n.birth_radius_max}}</li>
                                    <li>{format!{"birth_std: {}", n.birth_std}}</li>
                                    <li>{format!{"death_radius_max: {}", n.death_radius_max}}</li>
                                    <li>{format!{"death_std: {}", n.death_std}}</li>
                                </ul>
                            </div>
                }})
                .collect::<Vec<_>>()}
            </div>

        },
    }
}

#[component]
fn App() -> impl IntoView {
    // set up signals
    let (progress, set_progress) = create_signal::<f64>(0.0);
    let (max_t, set_max_t) = create_signal::<f64>(10.0);
    let (coords, set_coords) = create_signal::<Vec<SpeciesCoords>>(vec![]);
    let (history, set_history) = create_signal::<Vec<Checkpoint>>(vec![]);
    let (species_detail, set_species_detail) = create_signal(0);
    let (checked_species, set_checked_species) = create_signal::<Vec<usize>>(vec![]);
    let (heatmap, set_heatmap) = create_signal::<Vec<Vec<f64>>>(vec![]);

    // set up worker
    let worker = new_worker("worker");
    let onmessage = worker_onmessage(set_history, set_progress, set_coords, set_heatmap);
    worker.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();
    log!("app: worker created");

    // load species data
    let species_resource = create_resource(|| (), |_| async move { load_species().await });
    let chart_div_id = "plotly_chart".to_string();

    view! {
        <div id="backgroud" style="display:flex; background: #F0F8FF; flex-direction: row; justify-content: center; align-items: top">
            <div id ="main" style="width: 500px; background: white; padding: 50px; padding-top: 10px" >
                <h1  style="width: 500px">"Population dynamics simulation viewer"</h1>
                <h3  style="width: 500px">"Choose population parameters"</h3>
                <form style="width: 500px" on:submit=move |ev: leptos::ev::SubmitEvent| {
                    run_simulation(ev, species_resource, set_history, checked_species, max_t, worker.clone())
                }>
                    <div id="species" style="display:flex; flex-direction: row; justify-content: left; align-items: top">
                        <SpeciesSelector
                            species_resource=species_resource
                            species_detail=species_detail
                            set_species_detail=set_species_detail
                            checked_species=checked_species
                            set_checked_species=set_checked_species
                        />

                        <div id="details" style="border-style: solid; border-width: 1px; border-left-style: none; padding-right: 15px; display:flex; flex-direction: row; justify-content: left; align-items: top">
                            <SpeciesDetail0
                                species_resource=species_resource
                                species_detail=species_detail
                            />
                            <SpeciesDetail1
                                species_resource=species_resource
                                species_detail=species_detail
                            />
                        </div>
                    </div>
                    <h3>"Choose duration"</h3>
                    <div style="display:flex; flex-direction: row; gap: 10px; justify-content: left; align-items: top">
                        <form on:input=move |ev| {
                            ev.prevent_default();
                            set_max_t.set(event_target_value(&ev).parse::<f64>().unwrap());
                        }
                        >
                            <input type="number" id="max_t_selector" value=10 style="width: 50px"/>
                        </form>

                        <button type="submit" id="simulate_button">"Simulate"</button>
                        {move || view! {<progress id="simulation_progress" max={max_t.get()} value={progress.get()} />}}
                    </div>
                </form>
                <h3>"Viewer"</h3>
                <div  id="plotly_chart" style="width=500px"></div>
                <PlotlyChart div_id=chart_div_id.clone() size=(0.0, 1.0)/>
                {move || view! {<UpdateChart coords={coords.get()} div_id=chart_div_id.clone()/>}}
                <h3  style="width: 500px">"Replay"</h3>
                <form  style="width: 500px" on:input=move |ev| {
                    let view_idx = event_target_value(&ev).parse::<usize>().unwrap();
                    let history = history.get();
                    set_distribution(&history[view_idx], set_coords);
                }
                >
                    {move || view! {
                        <input
                            type="range"
                            min=0
                            max={
                                if !history.get().is_empty() {
                                    history.get().len() - 1
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
                <h3  style="width: 500px">"Heatmap"</h3>
                <div  id="plotly_heatmap" style="width=500px"></div>
                <PlotlyChart div_id={"plotly_heatmap".to_string()} size=(-0.5, 14.5)/>
                {move || view! {<UpdateHeatmap heatmap={heatmap.get()} div_id={"plotly_heatmap".to_string()} history={history.get()}/>}}
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}
