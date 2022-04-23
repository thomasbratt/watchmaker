use std::time::Duration;
// use crate::{Crossover, Evaluate, Initialize, Mutate};

pub struct Solver<G> {
    initialize: fn() -> G,
    evaluate: fn(G) -> f64,
    crossover: fn(G,G) -> G,
    mutate: fn(G) -> G,
    mutation_rate: f64,
    population_count: usize,
    time_limit: Duration,
}

impl <G> Solver<G> {
    pub fn solve(){

    }
}
