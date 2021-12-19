pub trait Mutate<G> {
    fn mutate(g: G) -> G;
}