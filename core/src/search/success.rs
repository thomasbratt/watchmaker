use crate::Reason;
use std::time::Duration;

/// Define the successful outcome of a genetic algorithm search.
#[derive(Clone, Debug, PartialEq)]
pub struct Success<G> {
    reason: Reason,
    epoch: usize,
    elapsed: Duration,
    best_cost: f64,
    mean_cost: f64,
    worst_cost: f64,
    best_genome: G,
}

impl<G> Success<G> {
    pub fn new(
        reason: Reason,
        epoch: usize,
        elapsed: Duration,
        best_cost: f64,
        mean_cost: f64,
        worst_cost: f64,
        best_genome: G,
    ) -> Self {
        Self {
            reason,
            epoch,
            elapsed,
            best_cost,
            mean_cost,
            worst_cost,
            best_genome,
        }
    }

    /// The reason the search terminated.
    pub fn reason(&self) -> Reason {
        self.reason
    }

    /// The number of epochs (iterations) the search took before terminating.
    pub fn epoch(&self) -> usize {
        self.epoch
    }

    /// The duration of the search.
    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }

    /// The cost of the best genome found by the search algorithm.
    /// This is the cost associated with [`Success::best_genome`].
    pub fn best_cost(&self) -> f64 {
        self.best_cost
    }

    /// The arithmetic mean cost of final generation of the algorithm.
    pub fn mean_cost(&self) -> f64 {
        self.mean_cost
    }

    /// The worst cost found in the final generation of the algorithm.
    pub fn worst_cost(&self) -> f64 {
        self.worst_cost
    }

    /// The best (lowest cost) genome found by the search algorithm.
    /// This is the genome associated with [`Success::best_cost`].
    pub fn best_genome(&self) -> &G {
        &self.best_genome
    }
}
