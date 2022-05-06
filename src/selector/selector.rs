use crate::Random;

pub trait Selector<G> {
    /// Select cross over partners, writing the indices of selected partners into `partner_indices`.
    ///
    /// # Arguments
    ///
    /// * `population` - The previous generation of genomes.  
    /// * `costs` - The cost associated with each item in `population`.
    /// * `random` - Syntax sugar for a source of randomness chosen at runtime.
    /// * `partner_indices` - The selected partners, as indices into `population`.
    ///
    fn select(
        &mut self,
        population: &[G],
        costs: &[f64],
        random: &mut Random,
        partner_indices: &mut [usize],
    );
}
