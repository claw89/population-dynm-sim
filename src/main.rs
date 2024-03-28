use csv;
use gloo::console::log;
use ndarray::{Array, Axis};
use simulate::*;
use yew::prelude::*;
use reqwest::get;



#[function_component(App)]
fn app() -> Html {
    let greeting_sate = use_state(|| "Hello, world!".to_string());
    {
        let greeting_sate = greeting_sate.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let greeting = get("http://codepi.local:8000/data/test.txt")
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                greeting_sate.set(greeting);
            });
            || ()
        }, ());
    }


    html! {
        <h1> { (*greeting_sate).clone() } </h1>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}

// fn main() {
//     let mut rdr = csv::Reader::from_path("data/species_params.csv").unwrap();

//     let species_array = Array::from_iter(
//         rdr.deserialize::<Species>()
//             .into_iter()
//             .map(|x| -> Species { x.unwrap() }),
//     );

//     let species_ids: Vec<usize> = vec![0, 1];

//     let mut population = Population::new(species_array.select(Axis(0), &species_ids).to_vec());
//     population.plot("images/initial.png");
//     population.simulate(31.0);
//     population.plot("images/final.png");
// }
