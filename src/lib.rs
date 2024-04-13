use rand::prelude::*;
use rand_distr::{Normal, WeightedIndex};
use serde::{Deserialize, Serialize};
use sprs::CsVec;
use std::f64::consts::PI;

#[derive(Serialize, Deserialize)]
pub struct WorkerMessageReceived {
    pub species_list: Vec<Species>,
    pub max_t: f64,
}

#[derive(Serialize, Deserialize)]
pub enum WorkerStatus {
    INITIALIZED,
    PENDING,
    COMPLETE,
}

#[derive(Serialize, Deserialize)]
pub struct WorkerResponse {
    pub status: WorkerStatus,
    pub checkpoint: Checkpoint,
}

/// Enumerates the possible events that can occur
#[derive(Clone, Copy)]
pub enum Event {
    /// An event in which a new individual is created
    Birth,
    /// An event in which an individual is destroyed
    Death,
    // Move,
}

/// A Species object holding the parameters that individuals of this species will use
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Species {
    pub id: usize,
    pub b0: f64,
    pub b1: f64,
    pub c1: f64,
    pub d0: f64,
    pub d1: f64,
    pub mbrmax: f64,
    pub mbsd: f64,
    pub mintegral: f64,
    pub move_radius_max: f64,
    pub move_std: f64,
    pub birth_radius_max: f64,
    pub birth_std: f64,
    birth_norm: Option<f64>,
    pub death_radius_max: f64,
    pub death_std: f64,
    death_norm: Option<f64>,
}

impl Species {
    /// Creates a new species; birth and death norms are calculated from the respective radius_max an std values
    pub fn new(
        id: usize,
        b0: f64,
        b1: f64,
        c1: f64,
        d0: f64,
        d1: f64,
        mbrmax: f64,
        mbsd: f64,
        mintegral: f64,
        move_radius_max: f64,
        move_std: f64,
        birth_radius_max: f64,
        birth_std: f64,
        death_radius_max: f64,
        death_std: f64,
    ) -> Self {
        let mut birth_norm = 0.0;
        if birth_std != 0.0 {
            birth_norm = 2.0
                * birth_std.powi(2)
                * PI
                * (1.0 - ((-1.0 * birth_radius_max.powi(2)) / (2.0 * birth_std.powi(2))).exp());
        }
        let mut death_norm = 0.0;
        if death_std != 0.0 {
            death_norm = 2.0
                * death_std.powi(2)
                * PI
                * (1.0 - ((-1.0 * death_radius_max.powi(2)) / (2.0 * death_std.powi(2))).exp());
        }
        Species {
            id,
            b0,
            b1,
            c1,
            d0,
            d1,
            mbrmax,
            mbsd,
            mintegral,
            move_radius_max,
            move_std,
            birth_radius_max,
            birth_std,
            birth_norm: Some(birth_norm),
            death_radius_max,
            death_std,
            death_norm: Some(death_norm),
        }
    }
}

/// An individual member of the population, which belongs to a species 
#[derive(Serialize, Deserialize, PartialEq, Clone)]
struct Individual {
    id: usize,
    species: Species,
    x_coord: f64,
    y_coord: f64,
    p_birth: f64,
    p_death: f64,
    p_move: f64,
    birth_neighbor_weight: f64,
    death_neighbor_weight: f64,
    birth_distances: CsVec<f64>,
    death_distances: CsVec<f64>,
}

impl Individual {
    pub fn new(id: usize, species: Species, x_coord: f64, y_coord: f64) -> Self {
        Individual {
            id,
            species,
            x_coord,
            y_coord,
            p_birth: 0.0,
            p_death: 0.0,
            p_move: 0.0,
            birth_neighbor_weight: 0.0,
            death_neighbor_weight: 0.0,
            birth_distances: CsVec::empty(1),
            death_distances: CsVec::empty(1),
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

        self.p_birth = self.species.b0 + self.birth_neighbor_weight;
        self.p_death = self.species.d0 + self.death_neighbor_weight;
        self.p_move = self.species.mintegral;
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Checkpoint {
    pub time: f64,
    // pub species_ids: Vec<usize>,
    // pub x_coords: Vec<f64>,
    // pub y_coords: Vec<f64>,
    pub species_individuals: Vec<(Vec<f64>, Vec<f64>)>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct History {
    pub checkpoints: Vec<Checkpoint>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Population {
    species_list: Vec<Species>,
    individuals: Vec<Individual>,
    pub size: usize,
    pub history: History,
    pub t: f64,
}

fn get_weight(distance: f64, var: f64, norm: f64) -> f64 {
    ((-1.0 * distance.powi(2)) / (2.0 * var)).exp() / norm
}

fn update_distances(distance: f64, individual: &mut Individual, event: Event, idx: usize) {
    match event {
        Event::Birth => {
            let radius = individual.species.birth_radius_max;
            let var = individual.species.birth_std.powi(2);
            let norm = individual.species.birth_norm;
            if distance < radius && var != 0.0 && norm.is_some() {
                individual
                    .birth_distances
                    .append(idx, get_weight(distance, var, norm.unwrap()));
            }
        }
        Event::Death => {
            let radius = individual.species.death_radius_max;
            let var = individual.species.death_std.powi(2);
            let norm = individual.species.death_norm;
            if distance < radius && var != 0.0 && norm.is_some() {
                individual
                    .death_distances
                    .append(idx, get_weight(distance, var, norm.unwrap()));
            }
        }
    }
}

impl Population {
    pub fn compute_initial_distances(&mut self) {
        let second_individuals = &self.individuals.clone();

        for first in &mut self.individuals {
            for second in second_individuals {
                if first.id != second.id {
                    let distance = first.distance(second);
                    update_distances(distance, first, Event::Birth, second.id);
                    update_distances(distance, first, Event::Death, second.id);
                }
            }
        }
    }

    pub fn new(species_list: Vec<Species>) -> Self {
        // create individuals for each species
        let mut individuals: Vec<Individual> = vec![];
        let mut idx = 0;
        let mut rng = rand::thread_rng();
        for species in species_list.clone() {
            for _ in 0..(species.c1 as usize) {
                let new_individual = Individual::new(idx, species, rng.gen(), rng.gen());
                individuals.push(new_individual);
                idx += 1;
            }
        }
        let initial_checkpoint = Checkpoint {
            time: 0.0,
            species_individuals: vec![] as Vec<(Vec<f64>, Vec<f64>)>,
        };

        // instantiate population
        Population {
            species_list,
            individuals,
            size: idx,
            history: History {
                checkpoints: vec![initial_checkpoint],
            },
            t: 0.0,
        }
    }

    fn compute_neighbor_weights(&mut self, event: &Event) {
        match event {
            Event::Birth => {
                for mut individual in self.individuals.clone() {
                    match individual.species.birth_norm {
                        Some(_) => {
                            individual.birth_neighbor_weight =
                                individual.birth_distances.data().iter().sum::<f64>()
                                    / individual.species.birth_norm.unwrap();
                        }
                        None => individual.birth_neighbor_weight = 0.0,
                    }
                }
            }
            Event::Death => {
                for mut individual in self.individuals.clone() {
                    match individual.species.death_norm {
                        Some(_) => {
                            individual.death_neighbor_weight =
                                individual.death_distances.data().iter().sum::<f64>()
                                    / individual.species.death_norm.unwrap();
                        }
                        None => individual.death_neighbor_weight = 0.0,
                    }
                }
            }
        }
    }

    fn update_probabilities(&mut self) {
        // update birth, death, and move probabilities
        for individual in self.individuals.iter_mut() {
            individual.update_probabilities();
        }
    }

    fn execute_birth(&mut self, parent: Individual) {
        // create a new invidual
        let parent = parent.clone();

        // initialise child position from parent with Gaussian kernel
        let mut rng = rand::thread_rng();
        let mut child_x_coord = Normal::new(parent.x_coord, parent.species.mbsd)
            .unwrap()
            .sample(&mut rng)
            % 1.0;
        if child_x_coord < 0.0 {
            child_x_coord += 1.0;
        }
        let mut child_y_coord = Normal::new(parent.y_coord, parent.species.mbsd)
            .unwrap()
            .sample(&mut rng)
            % 1.0;
        if child_y_coord < 0.0 {
            child_y_coord += 1.0;
        }

        let max_id = self.individuals.iter().map(|x| x.id).max().unwrap();
        let mut child = Individual::new(max_id + 1, parent.species, child_x_coord, child_y_coord);

        // initialize child distances and update other individuals
        for individual in &mut self.individuals {
            let distance = child.distance(individual);
            update_distances(distance, &mut child, Event::Birth, individual.id);
            update_distances(distance, &mut child, Event::Death, individual.id);

            update_distances(distance, individual, Event::Birth, child.id);
            update_distances(distance, individual, Event::Death, child.id);
        }

        // add child to vector of individuals
        self.individuals.push(child);
        self.size += 1;
    }

    fn execute_death(&mut self, deceased: Individual) {
        // remove an individual from the population
        let deceased_id = self
            .individuals
            .iter()
            .position(|x| *x == deceased)
            .unwrap();
        for individual in &mut self.individuals {
            let (birth_indices, birth_data): (Vec<usize>, Vec<f64>) = individual
                .birth_distances
                .iter()
                .filter(|(idx, _)| *idx != deceased_id)
                .unzip();
            individual.birth_distances = CsVec::new(self.size, birth_indices, birth_data);
            let (death_indices, death_data): (Vec<usize>, Vec<f64>) = individual
                .death_distances
                .iter()
                .filter(|(idx, _)| *idx != deceased_id)
                .unzip();
            individual.death_distances = CsVec::new(self.size, death_indices, death_data);
        }
        self.individuals.remove(deceased_id);
        self.size -= 1;
    }

    // fn execute_move<'b>(&'b mut self) {
    //     // move an individual within the population
    // }

    fn choose_event(&self) -> (Event, Individual, f64) {
        // pick the event type and individual at random from the poopulation
        let p_birth_sum = self.individuals.iter().fold(0.0, |acc, x| acc + x.p_birth);
        let p_death_sum = self.individuals.iter().fold(0.0, |acc, x| acc + x.p_death);
        let p_move_sum = self.individuals.iter().fold(0.0, |acc, x| acc + x.p_move);
        let p_total = p_birth_sum + p_death_sum + p_move_sum;

        let mut rng = rand::thread_rng();

        let choices = vec![Event::Birth, Event::Death, Event::Death];
        let weights = vec![
            p_birth_sum / p_total,
            p_death_sum / p_total,
            p_move_sum / p_total,
        ];
        let chosen_event = weighted_sample(&choices, &weights, &mut rng);

        let chosen_individual = match chosen_event {
            Event::Birth => {
                let weights = self
                    .individuals
                    .iter()
                    .map(|x| x.p_birth / p_birth_sum)
                    .collect();
                weighted_sample(&self.individuals.clone(), &weights, &mut rng)
            }
            Event::Death => {
                let weights = self
                    .individuals
                    .iter()
                    .map(|x| x.p_death / p_death_sum)
                    .collect();
                weighted_sample(&self.individuals.clone(), &weights, &mut rng)
            } // Event::Move => {
              //     let weights = self
              //         .individuals
              //         .iter()
              //         .map(|x| x.p_move / p_move_sum)
              //         .collect();
              //     weighted_sample(&self.individuals, &weights, &mut rng)
              // }
        };

        (chosen_event, chosen_individual, p_total)
    }

    fn get_checkpoint(&self) -> Checkpoint {
        let mut species_individuals = vec![] as Vec<(Vec<f64>, Vec<f64>)>;
        for species in self.species_list.clone() {
            let coords: Vec<(f64, f64)> = self
                .individuals
                .iter()
                .filter(|x| x.species.id == species.id)
                .map(|x| (x.x_coord, x.y_coord))
                .collect::<Vec<(f64, f64)>>();
            species_individuals.push(coords.into_iter().unzip());
        }
        Checkpoint {
            time: self.t,
            species_individuals,
        }
    }

    pub fn step(&mut self) -> (Checkpoint, f64) {
        for event in [Event::Birth, Event::Death] {
            self.compute_neighbor_weights(&event);
        }
        self.update_probabilities();

        let (chosen_event, chosen_individual_id, p_total) = self.choose_event();
        match chosen_event {
            Event::Birth => self.execute_birth(chosen_individual_id),
            Event::Death => self.execute_death(chosen_individual_id),
            // Event::Move => self.execute_move(),
        }
        (self.get_checkpoint(), p_total)
    }

    pub fn increment_time(&mut self, p_total: f64) {
        let mut rng = rand::thread_rng();
        let delta_t: f64 = (-1.0 / p_total) * (1.0 - rng.gen::<f64>()).ln();
        assert!(delta_t > 0.0);
        self.t += delta_t;
    }

    pub fn simulate(&mut self, max_t: f64) {
        while self.t < max_t {
            let (checkpoint, p_total) = self.step();
            self.increment_time(p_total);
            self.history.checkpoints.push(checkpoint);
        }
        println!("Completed with {:?} steps", self.history.checkpoints.len());
    }
}

fn weighted_sample<T>(choices: &[T], weights: &Vec<f64>, rng: &mut ThreadRng) -> T
where
    T: Clone,
{
    let dist = WeightedIndex::new(weights).unwrap();
    choices[dist.sample(rng)].clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn default_species() -> Species {
        Species::new(
            0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.1, 0.0, 0.0,
        )
    }

    #[rstest]
    fn test_new_individual(default_species: Species) {
        let individual = Individual::new(0, default_species, 0.0, 0.0);
        assert_eq!(individual.id, 0);
        assert_eq!(individual.species, default_species);
        assert_eq!(individual.x_coord, 0.0);
        assert_eq!(individual.y_coord, 0.0);
    }
}
