use crate::Reason;
use std::time::Duration;

// Define the successful outcome of a genetic algorithm search.
#[derive(Clone, Debug, PartialEq)]
pub struct Success<G> {
    pub reason: Reason,
    pub epoch: usize,
    pub elapsed: Duration,
    pub best_cost: f64,
    pub mean_cost: f64,
    pub worst_cost: f64,
    pub best_genome: G,
}
