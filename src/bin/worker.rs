use leptos::logging::log;
use population_dynm_sim::*;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{DedicatedWorkerGlobalScope, MessageEvent};

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
        population.compute_initial_distances();
        // population.simulate(received_message.max_t);
        while population.t < received_message.max_t {
            let (checkpoint, p_total) = population.step();
            population.increment_time(p_total);
            population.history.checkpoints.push(checkpoint.clone());

            // Post intermediate result
            let status = WorkerResponse {
                status: WorkerStatus::PENDING,
                checkpoint: checkpoint.clone(),
            };
            scope_clone
                .post_message(&serde_wasm_bindgen::to_value(&status).unwrap())
                .unwrap();
        }
        log!("worker: simulation complete");

        // Post final result
        let status = WorkerResponse {
            status: WorkerStatus::COMPLETE,
            checkpoint: Checkpoint {
                time: 0.0,
                // species_ids: vec![],
                // x_coords: vec![],
                // y_coords: vec![],
                species_individuals: vec![] as Vec<(Vec<f64>, Vec<f64>)>,
            },
        };
        scope_clone
            .post_message(&serde_wasm_bindgen::to_value(&status).unwrap())
            .unwrap();
    }) as Box<dyn Fn(MessageEvent)>);
    scope.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();

    let status = WorkerResponse {
        status: WorkerStatus::INITIALIZED,
        checkpoint: Checkpoint {
            time: 0.0,
            // species_ids: vec![],
            // x_coords: vec![],
            // y_coords: vec![],
            species_individuals: vec![] as Vec<(Vec<f64>, Vec<f64>)>,
        },
    };
    scope
        .post_message(&serde_wasm_bindgen::to_value(&status).unwrap())
        .unwrap();
}
