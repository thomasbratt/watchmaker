// Define operations on a generic chromosome, for use by genetic algorithm search.
pub trait Genetic<G> {
    fn initialize(&mut self) -> G;
    fn evaluate(&mut self, genome: &G) -> f64;
    fn crossover(&mut self, lhs: &G, rhs: &G) -> G;
    fn mutate(&mut self, original: &G) -> G;
}
