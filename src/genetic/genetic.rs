use std::fmt::Debug;

/// Define the genetic operations on a chromosome `G`.
pub trait Genetic<G>
where
    G: Clone + Debug + PartialEq + Send + Sync,
{
    /// Create a new, randomly initialized genome.
    fn initialize(&self) -> G;

    /// Evaluate the cost of the genome argument, with regards to a specific problem of interest.
    fn evaluate(&self, genome: &G) -> f64;

    /// Return a genome that is the random combination of the two supplied as arguments.
    fn crossover(&self, lhs: &G, rhs: &G) -> G;

    /// Return a randomly mutated copy of a genome.
    fn mutate(&self, genome: &G) -> G;
}
