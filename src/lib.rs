use itertools::{multiunzip, multizip, repeat_n, RepeatN};
use ndarray::{s, Array, Array2, ArrayBase, Axis, Dim, OwnedRepr};
use rand::prelude::*;
use rand_distr::{Normal, WeightedIndex};
use serde::{Deserialize, Serialize};
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

#[derive(Clone, Copy)]
pub enum Event {
    Birth,
    Death,
    // Move,
}

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
    pub death_radius_max: f64,
    pub death_std: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
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
    distances: ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>,
    pub history: History,
    pub t: f64,
}

impl Population {
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

        // compute initial distance matrix
        let mut distances = Array2::<f64>::ones((individuals.len(), individuals.len()));
        for first in &individuals {
            for seccond in &individuals {
                if first.id != seccond.id {
                    distances[[first.id, seccond.id]] = first.distance(seccond);
                }
            }
        }

        // initialise history
        // let history_vec: Vec<(usize, f64, f64)> = individuals
        //     .iter()
        //     .map(|x| (x.species.id, x.x_coord, x.y_coord))
        //     .collect();
        // let (species_ids, x_coords, y_coords) = multiunzip(history_vec);
        let initial_checkpoint = Checkpoint {
            time: 0.0,
            // species_ids,
            // x_coords,
            // y_coords,
            species_individuals: vec![] as Vec<(Vec<f64>, Vec<f64>)>,
        };

        // instantiate population
        Population {
            species_list,
            individuals,
            size: idx,
            distances,
            history: History {
                checkpoints: vec![initial_checkpoint],
            },
            t: 0.0,
        }
    }

    fn compute_neighbor_weights(&self, event: &Event) -> Vec<f64> {
        let distance_iter = self.distances.iter();

        let radius = self.individuals.iter().flat_map(|x| -> RepeatN<f64> {
            match event {
                Event::Birth => repeat_n(x.species.birth_radius_max, self.size),
                Event::Death => repeat_n(x.species.death_radius_max, self.size),
                // Event::Move => 0.0, // TODO
            }
        });
        let mask = distance_iter
            .clone()
            .zip(radius.clone())
            .map(|(d, r)| *d < r);
        let var = self.individuals.iter().flat_map(|x| -> RepeatN<f64> {
            match event {
                Event::Birth => repeat_n(x.species.birth_std.powi(2), self.size),
                Event::Death => repeat_n(x.species.death_std.powi(2), self.size),
                // Event::Move => 0.0, // TODO
            }
        });
        let effect = self.individuals.iter().map(move |x| -> f64 {
            match event {
                Event::Birth => x.species.b1,
                Event::Death => x.species.d1,
                // Event::Move => 0.0, // TODO
            }
        });

        let norm = radius.zip(var.clone()).map(|(r, v)| -> f64 {
            if v == 0.0 {
                0.0
            } else {
                2.0 * v * PI * (1.0 - ((-1.0 * r.powi(2)) / (2.0 * v)).exp())
            }
        });
        let weight_full = Array::from_iter(multizip((distance_iter, var, norm, mask)).map(
            |(d, v, n, m)| -> f64 {
                if v == 0.0 || n == 0.0 || !m {
                    0.0
                } else {
                    ((-1.0 * d.powi(2)) / (2.0 * v)).exp() / n
                }
            },
        ))
        .into_shape((self.size, self.size))
        .unwrap();
        let row_sum = weight_full.sum_axis(Axis(1));

        row_sum
            .into_iter()
            .zip(effect)
            .map(|(w, e)| w * e)
            .collect()
    }

    fn update_neighbor_weights(&mut self, weight: Vec<f64>, event: &Event) {
        // use the pairwise distances to update the individual neighbor weights

        for (i, w) in self.individuals.iter_mut().zip(weight) {
            match event {
                Event::Birth => i.birth_neighbor_weight = w,
                Event::Death => i.death_neighbor_weight = w,
                // Event::Move => (), // TODO
            }
        }
    }

    fn update_probabilities(&mut self) {
        // update birth, death, and move probabilities
        for individual in self.individuals.iter_mut() {
            individual.update_probabilities();
        }
    }

    fn execute_birth(&mut self, parent_id: usize) {
        // create a new invidual
        let parent = self.individuals[parent_id];

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
        let child = Individual::new(max_id + 1, parent.species, child_x_coord, child_y_coord);

        // update pairwise distance matrix
        let mut updated_distances = Array2::<f64>::ones((self.size + 1, self.size + 1));
        let child_distances = Array::from_iter(
            self.individuals
                .iter()
                .map(|i| -> f64 { i.distance(&child) }),
        );
        updated_distances
            .slice_mut(s![0..self.size, 0..self.size])
            .assign(&self.distances);
        updated_distances
            .slice_mut(s![self.size, 0..self.size])
            .assign(&child_distances);
        updated_distances
            .slice_mut(s![0..self.size, self.size])
            .assign(&child_distances);
        self.distances = updated_distances;

        // add child to vector of individuals
        self.individuals.push(child);
        self.size += 1;
    }

    fn execute_death(&mut self, deceased_id: usize) {
        // remove an individual from the population
        self.distances.remove_index(Axis(0), deceased_id);
        self.distances.remove_index(Axis(1), deceased_id);
        self.individuals.remove(deceased_id);
        self.size -= 1;
    }

    // fn execute_move<'b>(&'b mut self) {
    //     // move an individual within the population
    // }

    fn choose_event(&self) -> (Event, usize, f64) {
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
                weighted_sample(&self.individuals, &weights, &mut rng)
            }
            Event::Death => {
                let weights = self
                    .individuals
                    .iter()
                    .map(|x| x.p_death / p_death_sum)
                    .collect();
                weighted_sample(&self.individuals, &weights, &mut rng)
            } // Event::Move => {
              //     let weights = self
              //         .individuals
              //         .iter()
              //         .map(|x| x.p_move / p_move_sum)
              //         .collect();
              //     weighted_sample(&self.individuals, &weights, &mut rng)
              // }
        };
        let chosen_individual_id = self
            .individuals
            .iter()
            .position(|&x| x == chosen_individual)
            .unwrap();

        (chosen_event, chosen_individual_id, p_total)
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
        // let history_vec: Vec<(usize, f64, f64)> = self
        //     .individuals
        //     .iter()
        //     .map(|x| (x.species.id, x.x_coord, x.y_coord))
        //     .collect();
        // let (species_ids, x_coords, y_coords) = multiunzip(history_vec);
        Checkpoint {
            time: self.t,
            // species_ids,
            // x_coords,
            // y_coords,
            species_individuals,
        }
    }

    pub fn step(&mut self) -> (Checkpoint, f64) {
        for event in [Event::Birth, Event::Death] {
            let weights = self.compute_neighbor_weights(&event);
            self.update_neighbor_weights(weights, &event);
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
        //}, data_path: PathBuf, n_bins: usize) {
        // somulate the behaviour of the population over time
        // let mut t_prev: f64 = 0.0;
        // let prog = ProgressBar::new((max_t - 1.0) as u64);

        while self.t < max_t {
            // save the 2d histogram to data path
            // if t.floor() == t_prev.floor() + 1.0 {
            //     let hist = self.get_hist(n_bins);
            //     write_npy(
            //         data_path.join(format!("{:0>4}.npy", (t.floor() as u64))),
            //         &hist,
            //     )
            //     .unwrap();
            // }

            let (checkpoint, p_total) = self.step();
            // t_prev = t;
            self.increment_time(p_total);
            // if t as u64 > prog.position() + 1 {
            //     prog.inc(1);
            // }
            self.history.checkpoints.push(checkpoint);
        }
        // self.save_history();
        println!("Completed with {:?} steps", self.history.checkpoints.len());
    }

    // pub fn save_history(&self) {
    //     let mut file = File::create("data/history.json").unwrap();
    //     let _ = file.write_all(
    //         serde_json::to_string(&self.history.checkpoints)
    //             .expect("Expected checkpoint to be serialisable")
    //             .as_bytes(),
    //     );
    // }

    // fn get_hist(&self, n_bins: usize) -> ArrayBase<ndarray::OwnedRepr<f64>, Dim<[usize; 3]>> {
    //     let mut full_hist = Array3::<f64>::zeros((self.species_list.len(), n_bins + 2, n_bins + 2));
    //     // : ArrayBase<&f64>, Dim<[usize; &self.species_list.len()]>> = vec![];
    //     for (idx, species) in self.species_list.iter().enumerate() {
    //         let mut layer_hist = ndhistogram!(
    //             Uniform::new(n_bins, 0.0, 1.0),
    //             Uniform::new(n_bins, 0.0, 1.0)
    //         );
    //         for individual in self
    //             .individuals
    //             .iter()
    //             .filter(|individual| individual.species.id == species.id)
    //         {
    //             layer_hist.fill(&(individual.x_coord, individual.y_coord));
    //         }
    //         let species_layer: Array<f64, Dim<[usize; 1]>> =
    //             ArrayBase::from_iter(layer_hist.values().cloned());
    //         let species_layer_2d = species_layer
    //             .into_shape((n_bins + 2, n_bins + 2))
    //             .unwrap()
    //             .clone();
    //         full_hist
    //             .slice_mut(s![idx, 0..n_bins + 2, 0..n_bins + 2])
    //             .assign(&species_layer_2d);
    //         full_hist /= self.individuals.len() as f64;
    //     }
    //     full_hist
    //         .slice(s![0..self.species_list.len(), 1..n_bins + 1, 1..n_bins + 1])
    //         .to_owned()
    // }
}

fn weighted_sample<T>(choices: &[T], weights: &Vec<f64>, rng: &mut ThreadRng) -> T
where
    T: Copy,
{
    let dist = WeightedIndex::new(weights).unwrap();
    choices[dist.sample(rng)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn default_species() -> Species {
        Species {
            id: 0,
            b0: 0.0,
            b1: 0.0,
            c1: 0.0,
            d0: 0.0,
            d1: 0.0,
            mbrmax: 0.0,
            mbsd: 0.0,
            mintegral: 0.0,
            move_radius_max: 0.0,
            move_std: 0.0,
            birth_radius_max: 0.0,
            birth_std: 0.0,
            death_radius_max: 0.0,
            death_std: 0.0,
        }
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
