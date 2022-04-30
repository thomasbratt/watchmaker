use crate::common::Random;

// Define operations on a generic chromosome.
// For use by genetic algorithm search.
pub trait Genetic<G> {
    // fn new(random: &mut Random) -> Self;
    fn initialize(&mut self) -> G;
    fn evaluate(&mut self, genome: &G) -> f64;
    fn crossover(&mut self, lhs: &G, rhs: &G) -> G;
    fn mutate(&mut self, original: &G) -> G;
    fn random(&mut self) -> &mut Random;
}
