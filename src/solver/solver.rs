use crate::{Crossover, Evaluate, Initialize, Mutate};

pub struct Solver<G> {
    initialize: Initialize<G>,
    evaluate: Evaluate<G>,
    crossover: Crossover<G>,
    mutate: Mutate<G>,
    mutationRate: f64,
    populationCount: usize,
    timeLimit: TimeSpan,
}

impl Solver<G> {
    pub fn solve(){

    }
}
