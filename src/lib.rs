use ndarray::{s, Array, Array2, Axis};
use rand::prelude::*;
use rand_distr::Normal;
use std::f64::consts::PI;

enum Event {
    Birth,
    Death,
    Move,
}


#[derive(PartialEq, Debug)]
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

#[derive(PartialEq)]
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

    fn execute_birth(&mut self, parent: &Individual<'a>) {
        // create a new invidual

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

    fn execute_death(&mut self, deceased: &Individual) {
        // remove an individual from the population
        let deceased_id = self.individuals.iter().position(|x| x == deceased).unwrap();
        self.distances.remove_index(Axis(0), deceased_id);
        self.distances.remove_index(Axis(1), deceased_id);
        self.individuals.remove(deceased_id);
        self.size -= 1;
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


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{rstest, fixture};

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