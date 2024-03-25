use simulate::*;

fn main() {
    let species = Species {
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
    };

    let mut population = Population::new(vec![&species]);
    population.simulate(101.0);
}