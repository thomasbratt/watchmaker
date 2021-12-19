pub trait Crossover<G> {
    fn crossover(lhs: G, rhs: G) -> G;
}