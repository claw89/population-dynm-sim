use itertools::Itertools;
use simulate::*;
use leptos::*;
use reqwest;


async fn load_species() -> Vec<Species> {
    let species_bytes = reqwest::get("http://codepi.local:8000/data/species_params.csv").await.unwrap().text().await.unwrap();
    let mut rdr = csv::ReaderBuilder::new().delimiter(b',').from_reader(species_bytes.as_bytes());
    rdr.deserialize::<Species>()
        .into_iter()
        .map(|x| -> Species { x.unwrap() }).collect_vec()
}


#[component]
fn App() -> impl IntoView {
    let species_resource = create_resource(|| (), |_| async move { load_species().await });

    view! {
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
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> } );
}
