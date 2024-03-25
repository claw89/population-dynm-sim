use ndarray::{s, Array, Array2, Axis};
use rand::prelude::*;
use rand_distr::{Normal, WeightedIndex};
use std::f64::consts::PI;

#[derive(Clone, Copy)]
enum Event {
    Birth,
    Death,
    Move,
}

#[derive(PartialEq, Debug)]
pub struct Species {
    pub id: u8,
    pub B0: f64,
    pub B1: f64,
    pub C1: f64,
    pub D0: f64,
    pub D1: f64,
    pub Mbrmax: f64,
    pub Mbsd: f64,
    pub Mintegral: f64,
    pub Mrmax: f64,
    pub Msd: f64,
    pub Wbrmax: f64,
    pub Wbsd: f64,
    pub Wdrmax: f64,
    pub Wdsd: f64,
}

#[derive(PartialEq, Clone, Copy)]
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

pub struct Population<'a> {
    individuals: Vec<Individual<'a>>,
    size: usize,
    distances: Array2<f64>,
    // history
}

impl<'a> Population<'a> {
    pub fn new(species_list: Vec<&'a Species>) -> Self {
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

    fn update_neighbor_weights<'b>(&'b mut self, event: Event) {
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

    fn update_probabilities<'b>(&'b mut self) {
        // update birth, death, and move probabilities
        for mut individual in self.individuals.iter_mut() {
            individual.update_probabilities();
        }
    }

    fn execute_birth<'b>(&'b mut self, parent_id: usize) {
        // create a new invidual
        let parent = self.individuals[parent_id];

        // initialise child position from parent with Gaussian kernel
        let mut rng = rand::thread_rng();
        let child_x_coord = Normal::new(parent.x_coord, parent.species.Mbsd)
            .unwrap()
            .sample(&mut rng);
        let child_y_coord = Normal::new(parent.y_coord, parent.species.Mbsd)
            .unwrap()
            .sample(&mut rng);

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

    fn execute_move<'b>(&'b mut self) {
        // move an individual within the population
    }

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
            }
            Event::Move => {
                let weights = self
                    .individuals
                    .iter()
                    .map(|x| x.p_move / p_move_sum)
                    .collect();
                weighted_sample(&self.individuals, &weights, &mut rng)
            }
        };
        let chosen_individual_id = self
            .individuals
            .iter()
            .position(|&x| x == chosen_individual)
            .unwrap();

        (chosen_event, chosen_individual_id, p_total)
    }

    pub fn simulate(&'a mut self, max_t: f64) {
        // somulate the behaviour of the population over time
        let mut t: f64 = 0.0;
        let mut rng = rand::thread_rng();

        while t < max_t {
            for event in [Event::Birth, Event::Death] {
                self.update_neighbor_weights(event);
            }
            self.update_probabilities();

            let (chosen_event, chosen_individual_id, p_total) = self.choose_event();
            match chosen_event {
                Event::Birth => self.execute_birth(chosen_individual_id),
                Event::Death => self.execute_death(chosen_individual_id),
                Event::Move => self.execute_move(),
            }
            let delta_t: f64 = (-1.0 / p_total) * (1.0 - rng.gen::<f64>()).ln();
            assert!(delta_t > 0.0);
            t += delta_t;
        }
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
            B0: 0.0,
            B1: 0.0,
            C1: 0.0,
            D0: 0.0,
            D1: 0.0,
            Mbrmax: 0.0,
            Mbsd: 0.0,
            Mintegral: 0.0,
            Mrmax: 0.0,
            Msd: 0.0,
            Wbrmax: 0.0,
            Wbsd: 0.0,
            Wdrmax: 0.0,
            Wdsd: 0.0,
        }
    }

    #[rstest]
    fn test_new_individual(default_species: Species) {
        let individual = Individual::new(0, &default_species, 0.0, 0.0);
        assert_eq!(individual.id, 0);
        assert_eq!(individual.species, &default_species);
        assert_eq!(individual.x_coord, 0.0);
        assert_eq!(individual.y_coord, 0.0);
    }
}
