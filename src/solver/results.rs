use crate::solver::stopping_criterion::StoppingCriterion;
use std::time::Duration;

// The results of the genetic algorithm search.
#[derive(Debug)]
pub struct Results<G> {
    pub reason: StoppingCriterion,
    pub epoch: usize,
    pub elapsed: Duration,
    pub best_cost: f64,
    pub mean_cost: f64,
    pub worst_cost: f64,
    pub best_genome: G,
}
