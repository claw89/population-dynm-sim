use ndarray::{Array, Array1, Array2, Axis};
use rand::prelude::*;

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
    birth_neighbors: u32,
    death_neighbors: u32,
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
            birth_neighbors: 0,
            death_neighbors: 0,
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

    fn get_pairwise_distances() {
        // compute the pairwise distances for all individuals in the population
    }

    fn update_neighbor_weights(&self) {
        // use the pairwise distances to update the individual neighbor weights

        let Wbrmax_iter = self.individuals.iter().map(|x| x.species.Wbrmax);
        let birth_r_max = Array::from_iter(Wbrmax_iter)
            .into_shape((self.size, 1))
            .unwrap();
        let birth_neighbors: Array1<u32> = (&self.distances - &birth_r_max)
            .map(|x| (*x < 0.0) as u32)
            .sum_axis(Axis(1));

        // let birth_std: Vec<f64> = self.individuals.iter().map(|x| x.species.Wbsd).collect();
        // let birth_effect =  self.individuals.iter().map(|x| x.species.B1).collect();

        // if event_type == "birth":
        //     rmax = np.array([i.species.Wbrmax for i in self.individuals])
        //     std = np.array([i.species.Wbsd for i in self.individuals])
        //     effect_param = [i.species.B1 for i in self.individuals]
        // elif event_type == "death":
        //     rmax = np.array([i.species.Wdrmax for i in self.individuals])
        //     std = np.array([i.species.Wdsd for i in self.individuals])
        //     effect_param = [i.species.D1 for i in self.individuals]
        // else:
        //     raise ValueError

        // neighbors = (pairwise_distances < rmax).sum(axis=1)

        // var = std ** 2
        // norm = np.where(var != 0, 2 * np.pi * var * (1 - np.exp(-rmax**2 / (2 * var))), 0)
        // weights = np.where(
        //     pairwise_distances < rmax,
        //     np.where(var != 0, np.exp((-1 * pairwise_distances ** 2) / (2 * var)) / norm, 0),
        //     0
        // ).sum(axis=1) * effect_param

        // if event_type == "birth":
        //     for i, individual in enumerate(self.individuals):
        //         individual.birth_neighbors = neighbors[i]
        //         individual.birth_neighbor_weight = weights[i]
        // elif event_type == "death":
        //     for i, individual in enumerate(self.individuals):
        //         individual.death_neighbors = neighbors[i]
        //         individual.death_neighbor_weight = weights[i]
    }

    fn update_probabilities() {
        // update birth, death, and move probabilities
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
