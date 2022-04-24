use rand::RngCore;

// Define operations on a generic chromosome.
// For use by genetic algorithm search.
pub trait Genetic<G> {
    fn initialize(random: &mut Box<dyn RngCore>) -> G;
    fn evaluate(genome: G) -> f64;
    fn crossover(random: &mut Box<dyn RngCore>, lhs: G, rhs: G) -> G;
    fn mutate(random: &mut Box<dyn RngCore>, original: G) -> G;
}
