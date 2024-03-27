use indicatif::ProgressBar;
use itertools::{multiunzip, multizip, repeat_n, RepeatN};
use ndarray::{s, Array, Array2, Axis};
use plotters::prelude::*;
use rand::prelude::*;
use rand_distr::{Normal, WeightedIndex};
use serde::{Deserialize, Serialize};
use serde_json;
use std::f64::consts::PI;
use std::fs::File;
use std::io::prelude::*;

const COLORS: [RGBColor; 5] = [
    full_palette::BLUE_600,
    full_palette::ORANGE_600,
    full_palette::GREEN_600,
    full_palette::RED_600, 
    full_palette::PURPLE_600
];

#[derive(Clone, Copy)]
enum Event {
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

#[derive(PartialEq, Clone, Copy)]
struct Individual {
    id: usize,
    species: Species,
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

impl Individual {
    pub fn new(id: usize, species: Species, x_coord: f64, y_coord: f64) -> Self {
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

        self.p_birth = self.species.b0 + self.birth_neighbor_weight;
        self.p_death = self.species.d0 + self.death_neighbor_weight;
        self.p_move = self.species.mintegral;
    }
}

#[derive(Serialize, Deserialize)]
struct Checkpoint {
    time: f64,
    species_ids: Vec<usize>,
    x_coords: Vec<f64>,
    y_coords: Vec<f64>,
}

#[derive(Serialize, Deserialize)]
struct History {
    checkpoints: Vec<Checkpoint>,
}

pub struct Population {
    individuals: Vec<Individual>,
    size: usize,
    distances: Array2<f64>,
    history: History,
}

impl Population {
    pub fn new(species_list: Vec<Species>) -> Self {
        // create individuals for each species
        let mut individuals: Vec<Individual> = vec![];
        let mut idx = 0;
        let mut rng = rand::thread_rng();
        for species in species_list {
            for _ in 0..(species.c1 as usize) {
                let new_individual = Individual::new(idx, species.clone(), rng.gen(), rng.gen());
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

        // initialise history
        let history_vec: Vec<(usize, f64, f64)> = individuals
            .iter()
            .map(|x| (x.species.id, x.x_coord, x.y_coord))
            .collect();
        let (species_ids, x_coords, y_coords) = multiunzip(history_vec);
        let initial_checkpoint = Checkpoint {
            time: 0.0,
            species_ids,
            x_coords,
            y_coords,
        };

        // instantiate population
        Population {
            individuals: individuals,
            size: idx,
            distances: distances,
            history: History {
                checkpoints: vec![initial_checkpoint],
            },
        }
    }

    fn compute_neighbor_weights(&self, event: &Event) -> Vec<f64> {
        let distance_iter = self.distances.iter();

        let radius = self
            .individuals
            .iter()
            .map(|x| -> RepeatN<f64> {
                match event {
                    Event::Birth => repeat_n(x.species.birth_radius_max, self.size),
                    Event::Death => repeat_n(x.species.death_radius_max, self.size),
                    // Event::Move => 0.0, // TODO
                }
            })
            .flatten();
        let mask = distance_iter
            .clone()
            .zip(radius.clone())
            .map(|(d, r)| *d < r);
        let var = self
            .individuals
            .iter()
            .map(|x| -> RepeatN<f64> {
                match event {
                    Event::Birth => repeat_n(x.species.birth_std.powi(2), self.size),
                    Event::Death => repeat_n(x.species.death_std.powi(2), self.size),
                    // Event::Move => 0.0, // TODO
                }
            })
            .flatten();
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
                if v == 0.0 || n == 0.0 || m == false {
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

    fn update_neighbor_weights<'b>(&'b mut self, weight: Vec<f64>, event: &Event) {
        // use the pairwise distances to update the individual neighbor weights

        for (i, w) in self.individuals.iter_mut().zip(weight) {
            match event {
                Event::Birth => i.birth_neighbor_weight = w,
                Event::Death => i.death_neighbor_weight = w,
                // Event::Move => (), // TODO
            }
        }
    }

    fn update_probabilities<'b>(&'b mut self) {
        // update birth, death, and move probabilities
        for individual in self.individuals.iter_mut() {
            individual.update_probabilities();
        }
    }

    fn execute_birth<'b>(&'b mut self, parent_id: usize) {
        // create a new invidual
        let parent = self.individuals[parent_id];

        // initialise child position from parent with Gaussian kernel
        let mut rng = rand::thread_rng();
        let mut child_x_coord = Normal::new(parent.x_coord, parent.species.mbsd)
            .unwrap()
            .sample(&mut rng) % 1.0;
        if child_x_coord < 0.0 {
            child_x_coord += 1.0;
        }
        let mut child_y_coord = Normal::new(parent.y_coord, parent.species.mbsd)
            .unwrap()
            .sample(&mut rng) % 1.0;
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
            .slice_mut(s![0..-1, 0..-1])
            .assign(&self.distances);
        updated_distances
            .slice_mut(s![-1, 0..-1])
            .assign(&child_distances);
        updated_distances
            .slice_mut(s![0..-1, -1])
            .assign(&child_distances);
        self.distances = updated_distances;

        // add child to vector of individuals
        self.individuals.push(child);
        self.size += 1;
    }

    fn execute_death<'b>(&'b mut self, deceased_id: usize) {
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

    fn get_checkpoint(&self, time: f64) -> Checkpoint {
        let history_vec: Vec<(usize, f64, f64)> = self
            .individuals
            .iter()
            .map(|x| (x.species.id, x.x_coord, x.y_coord))
            .collect();
        let (species_ids, x_coords, y_coords) = multiunzip(history_vec);
        Checkpoint {
            time,
            species_ids,
            x_coords,
            y_coords,
        }
    }

    pub fn simulate(&mut self, max_t: f64) {
        // somulate the behaviour of the population over time
        let mut t: f64 = 0.0;
        let mut rng = rand::thread_rng();
        let prog = ProgressBar::new((max_t - 1.0) as u64);

        while t < max_t {
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
            let next_checkpoint = self.get_checkpoint(t);
            self.history.checkpoints.push(next_checkpoint);
            let delta_t: f64 = (-1.0 / p_total) * (1.0 - rng.gen::<f64>()).ln();
            assert!(delta_t > 0.0);
            t += delta_t;
            if t as u64 > prog.position() + 1 {
                prog.inc(1);
            }
        }
        self.save_history();
        println!("Completed with {:?} steps", self.history.checkpoints.len());
    }

    pub fn save_history(&self) {
        let mut file = File::create("data/history.json").unwrap();
        let _ = file.write_all(
            serde_json::to_string(&self.history.checkpoints)
                .expect("Expected checkpoint to be serialisable")
                .as_bytes(),
        );
    }

    pub fn plot(&self, path: &str) {
        let drawing_area = BitMapBackend::new(path, (768, 768)).into_drawing_area();

        drawing_area.fill(&WHITE).unwrap();

        let mut ctx = ChartBuilder::on(&drawing_area)
            .margin(35)
            .build_cartesian_2d(0.0f64..1.0f64, 0.0f64..1.0f64)
            .unwrap();

        ctx.configure_mesh().max_light_lines(0).draw().unwrap();

        ctx.draw_series(
            self.individuals
                .iter()
                .map(|x| Circle::new((x.x_coord, x.y_coord), 8, COLORS[x.species.id].mix(0.6).filled())),
        )
        .unwrap();
    }
}

fn weighted_sample<T>(choices: &Vec<T>, weights: &Vec<f64>, rng: &mut ThreadRng) -> T
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
