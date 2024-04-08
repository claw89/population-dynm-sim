use leptos::logging::log;
use population_dynm_sim::{Population, Species};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{DedicatedWorkerGlobalScope, MessageEvent};

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

fn main() {
    console_error_panic_hook::set_once();
    log!("worker: starting");

    let scope = DedicatedWorkerGlobalScope::from(JsValue::from(js_sys::global()));
    let scope_clone = scope.clone();
    let onmessage = Closure::wrap(Box::new(move |msg: MessageEvent| {
        log!("worker: received message");

        let received_message: WorkerMessageReceived =
            serde_wasm_bindgen::from_value(msg.data()).unwrap();
        log!("worker: simulating");
        let mut population = Population::new(received_message.species_list);
        population.simulate(received_message.max_t);
        log!("worker: simulation complete");

        let status = WorkerResponse {
            status: WorkerStatus::COMPLETE,
            population_size: population.size,
        };
        scope_clone
            .post_message(&serde_wasm_bindgen::to_value(&status).unwrap())
            .unwrap();
    }) as Box<dyn Fn(MessageEvent)>);
    scope.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();

    let status = WorkerResponse {
        status: WorkerStatus::INITIALIZED,
        population_size: 0,
    };
    scope
        .post_message(&serde_wasm_bindgen::to_value(&status).unwrap())
        .unwrap();
}
