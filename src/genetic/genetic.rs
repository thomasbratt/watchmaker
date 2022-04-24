use crate::common::Random;

// Define operations on a generic chromosome.
// For use by genetic algorithm search.
pub trait Genetic<G> {
    fn initialize(random: &mut Random) -> G;
    fn evaluate(genome: G) -> f64;
    fn crossover(random: &mut Random, lhs: G, rhs: G) -> G;
    fn mutate(random: &mut Random, original: G) -> G;
}
