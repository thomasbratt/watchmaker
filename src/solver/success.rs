use crate::solver::reason::Reason;
use std::time::Duration;

// Define the successful outcome of a genetic algorithm search.
#[derive(Debug)]
pub struct Success<G> {
    pub reason: Reason,
    pub epoch: usize,
    pub elapsed: Duration,
    pub best_cost: f64,
    pub mean_cost: f64,
    pub worst_cost: f64,
    pub best_genome: G,
}
