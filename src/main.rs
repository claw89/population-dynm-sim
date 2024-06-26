use ndarray::{Array, Array1, Array2, Axis};
use rand::prelude::*;
use std::f64::consts::PI;

enum Event {
    Birth,
    Death,
    Move,
}

struct Species {
    id: u8,
    B0: f64,
    B1: f64,
    C1: f64,
    D0: f64,
    D1: f64,
    Mbrmax: f64,
    Mbsd: f64,
    Mintegral: f64,
    Mrmax: f64,
    Msd: f64,
    Wbrmax: f64,
    Wbsd: f64,
    Wdrmax: f64,
    Wdsd: f64,
}

struct Individual<'a> {
    id: usize,
    species: &'a Species,
    x_coord: f64,
    y_coord: f64,
    p_birth: f64,
    p_death: f64,
    p_move: f64,
    // birth_neighbors: u32,
    // death_neighbors: u32,
    birth_neighbor_weight: f64,
    death_neighbor_weight: f64,
}

impl<'a> Individual<'a> {
    pub fn new(id: usize, species: &'a Species, x_coord: f64, y_coord: f64) -> Self {
        Individual {
            id: id,
            species: species,
            x_coord: x_coord,
            y_coord: y_coord,
            p_birth: 0.0,
            p_death: 0.0,
            p_move: 0.0,
            // birth_neighbors: 0,
            // death_neighbors: 0,
            birth_neighbor_weight: 0.0,
            death_neighbor_weight: 0.0,
        }
    }

    pub fn distance(&self, other: &Individual) -> f64 {
        // Compute the Euclidean distance between the positions of two individuals

        let inside_delta_x = (self.x_coord - other.x_coord).abs();
        let delta_x = inside_delta_x.min(1.0 - inside_delta_x);

        let inside_delta_y = (self.y_coord - other.y_coord).abs();
        let delta_y = inside_delta_y.min(1.0 - inside_delta_y);

        (delta_x.powi(2) + delta_y.powi(2)).sqrt()
    }

    pub fn update_probabilities(&mut self) {
        // Update individual birth, death, and move probabilities

        self.p_birth = self.species.B0 + self.birth_neighbor_weight;
        self.p_death = self.species.D0 + self.death_neighbor_weight;
        self.p_move = self.species.Mintegral;
    }
}

struct Population<'a> {
    individuals: Vec<Individual<'a>>,
    size: usize,
    distances: Array2<f64>,
    // history
}

impl<'a> Population<'a> {
    fn new(&self, species_list: Vec<&'a Species>) -> Self {
        // create individuals for each species
        let mut individuals: Vec<Individual> = vec![];
        let mut idx = 0;
        let mut rng = rand::thread_rng();
        for species in species_list {
            for _ in 0..(species.C1 as usize) {
                let new_individual = Individual::new(idx, species, rng.gen(), rng.gen());
                individuals.push(new_individual);
                idx += 1;
            }
        }

        // compute initial distance matrix
        let mut distances = Array2::<f64>::ones((individuals.len(), individuals.len()));
        for first in &individuals {
            for seccond in &individuals {
                if first.id != seccond.id {
                    distances[[first.id, seccond.id]] = first.distance(&seccond);
                }
            }
        }

        // instantiate population
        Population {
            individuals: individuals,
            size: idx,
            distances: distances,
        }
    }

    fn update_neighbor_weights(&mut self, event: Event) {
        // use the pairwise distances to update the individual neighbor weights

        let radius = Array::from_iter(self.individuals.iter().map(|x| -> f64 {
            match event {
                Event::Birth => x.species.Wbrmax,
                Event::Death => x.species.Wdrmax,
                Event::Move => 0.0, // TODO
            }
        }))
        .into_shape((self.size, 1))
        .unwrap();
        let mask = (&self.distances - &radius).map(|x| *x < 0.0);
        let var = Array::from_iter(self.individuals.iter().map(|x| -> f64 {
            match event {
                Event::Birth => x.species.Wbsd.powi(2),
                Event::Death => x.species.Wdsd.powi(2),
                Event::Move => 0.0, // TODO
            }
        }));
        let effect = self.individuals.iter().map(|x| -> f64 {
            match event {
                Event::Birth => x.species.B1,
                Event::Death => x.species.D1,
                Event::Move => 0.0, // TODO
            }
        });

        let norm = Array::from_iter(radius.iter().zip(var.iter()).map(|(r, v)| -> f64 {
            if *v == 0.0 {
                0.0
            } else {
                2.0 * v * PI * (1.0 - ((-1.0 * r.powi(2)) / (2.0 * v)).exp())
            }
        }));

        let weight = Array::from_iter(
            Array::from_iter(
                self.distances
                    .iter()
                    .zip(var.iter())
                    .zip(norm.iter())
                    .zip(mask.iter())
                    .into_iter()
                    .map(|(((d, v), n), m)| -> f64 {
                        if *v == 0.0 || *n == 0.0 || *m == false {
                            0.0
                        } else {
                            ((-1.0 * d.powi(2)) / (2.0 * v)).exp() / n
                        }
                    }),
            )
            .sum_axis(Axis(1))
            .into_iter()
            .zip(effect)
            .map(|(w, e)| w * e),
        );

        for (w, i) in weight.iter().zip(self.individuals.iter_mut()) {
            match event {
                Event::Birth => i.birth_neighbor_weight = *w,
                Event::Death => i.death_neighbor_weight = *w,
                Event::Move => (), // TODO
            }
        }
    }

    fn update_probabilities(&mut self) {
        // update birth, death, and move probabilities
        for mut individual in self.individuals.iter_mut() {
            individual.update_probabilities();
        }
    }

    fn execute_birth() {
        // create a new invidual
    }

    fn execute_death() {
        // remove an individual from the population
    }

    fn execute_move() {
        // move an individual within the population
    }

    fn choose_event() {
        // pick the event type and individual at random from the poopulation
    }

    pub fn simulate() {
        // somulate the behaviour of the population over time
    }
}

fn main() {
    println!("Hello world")
}
