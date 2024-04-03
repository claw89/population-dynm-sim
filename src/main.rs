use ndarray::{Array, Axis};
use simulate::*;
use std::time::Instant;

fn main() {
    let mut rdr = csv::Reader::from_path("data/species_params.csv").unwrap();

    let species_array = Array::from_iter(
        rdr.deserialize::<Species>()
            .map(|x| -> Species { x.unwrap() }),
    );

    let species_ids: Vec<usize> = vec![0, 1];
    let now = Instant::now();
    let mut population = Population::new(species_array.select(Axis(0), &species_ids).to_vec());
    population.plot("images/initial.png");
    population.simulate(30.0);
    population.plot("images/final.png");
    let elapsed_time = now.elapsed();
    println!("Completed in {}", elapsed_time.as_secs());
}
