use std::time::Duration;

/// Define the progress reporting callback.
///
/// Implementors should return true to continue the search and false to terminate it.
pub type Progress<G> = Box<dyn FnMut(ProgressSnapshot<G>)>;

/// Define a snapshot of the search progress.
#[derive(Clone, Debug, PartialEq)]
pub struct ProgressSnapshot<'a, G> {
    epoch: usize,
    elapsed: Duration,
    best_cost: f64,
    best_genome: &'a G,
}

impl<'a, G> ProgressSnapshot<'a, G> {
    pub fn new(epoch: usize, elapsed: Duration, best_cost: f64, best_genome: &'a G) -> Self {
        Self {
            epoch,
            elapsed,
            best_cost,
            best_genome,
        }
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

    /// The best (lowest cost) genome found by the search algorithm.
    /// This is the genome associated with [`Success::best_cost`].
    pub fn best_genome(&self) -> &G {
        self.best_genome
    }
}
