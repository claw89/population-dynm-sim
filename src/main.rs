

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
    id: u8,
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

impl <'a> Individual<'a> {

    pub fn new(id: u8, species: &'a Species, x_coord: f64, y_coord: f64) ->Self {
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

    pub fn distance(&self, other: &Individual) {
        // Compute the Euclidean distance between the positions of two individuals
    }
    
    pub fn update_probabilities() {
        // uupdate individual birth, death, and move probabilities
    }
}

struct Population<'a> {
    individuals: Vec<&'a Individual<'a>>
    // history
}

impl Population<'_> {
    fn get_pairwise_distances() {
        // compute the pairwise distances for all individuals in the population
    }

    fn update_neighbor_weights() {
        // use the pairwise distances to update the individual neighbor weights
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