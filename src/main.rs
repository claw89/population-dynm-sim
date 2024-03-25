use simulate::*;

fn main() {
    let species = Species {
        id: 0,
        B0: 0.4,
        B1: 0.0,
        C1: 20.0,
        D0: 0.2,
        D1: 0.001,
        Mbrmax: 0.06,
        Mbsd: 0.02,
        Mintegral: 0.0,
        Mrmax: 0.0,
        Msd: 0.0,
        Wbrmax: 0.0,
        Wbsd: 0.0,
        Wdrmax: 0.36,
        Wdsd: 0.12,
    };

    let mut population = Population::new(vec![&species]);
    population.simulate(21.0);
}
