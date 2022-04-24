use crate::Genetic;
use crate::solver::stopping_criterion::StoppingCriterion;
use std::time::Duration;

// The results of the genetic algorithm search.
pub struct Results<G> where G: Genetic<G> {
    pub reason: StoppingCriterion,
    pub epoch_count: u64,
    pub duration: Duration,
    pub best_cost: f64,
    pub mean_cost: f64,
    pub worst_cost: f64,
    pub best_genome: G,
}