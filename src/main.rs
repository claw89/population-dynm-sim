use chrono::Utc;
use ndarray::{Array, Axis};
use simulate::*;
use std::fs;
use std::path::Path;
use std::time::Instant;

fn main() {
    if !Path::new("./data").exists() {
        fs::create_dir("./data").unwrap();
    }
    let now = Utc::now();
    let mut data_path = String::from("./data/");
    data_path.push_str(&now.format("%Y-%m-%d_%H:%M:%S").to_string());
    fs::create_dir(&data_path).unwrap();

    let mut rdr = csv::Reader::from_path("data/species_params.csv").unwrap();
    let species_array = Array::from_iter(
        rdr.deserialize::<Species>()
            .map(|x| -> Species { x.unwrap() }),
    );

    let species_ids: Vec<usize> = vec![0, 1];
    let now = Instant::now();
    let mut population = Population::new(species_array.select(Axis(0), &species_ids).to_vec());
    // population.plot("images/initial.png");
    population.simulate(10.0, Path::new(&data_path));
    // population.plot("images/final.png");
    let elapsed_time = now.elapsed();
    println!("Completed in {}", elapsed_time.as_secs());
}
