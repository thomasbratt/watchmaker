use crate::selector::Selector;
use crate::Failure;
use rand::Rng;
use std::marker::PhantomData;

#[derive(Clone, Debug, PartialEq)]
pub struct TournamentSelector<G> {
    cross_over_candidates: usize,
    _phantom_data: PhantomData<G>,
}

impl<G> TournamentSelector<G> {
    /// Create new Tournament Population Selector.
    ///
    /// # Arguments
    ///
    /// * `cross_over_candidates` - The number of candidate genomes that will be compared when
    ///                             deciding which genome to use when new genomes are generated
    ///                             during cross over.  
    pub fn new(cross_over_candidates: usize) -> Result<Self, Failure> {
        if cross_over_candidates < 1 {
            return Err(Failure::cross_over_candidates());
        }
        Ok(Self {
            cross_over_candidates,
            _phantom_data: Default::default(),
        })
    }

    /// The number of candidate genomes that will be compared when deciding which genome to
    /// use when new genomes are generated during cross over.  
    pub fn cross_over_candidates(&self) -> usize {
        self.cross_over_candidates
    }
}

impl<G> Default for TournamentSelector<G> {
    fn default() -> Self {
        Self {
            cross_over_candidates: 8,
            _phantom_data: Default::default(),
        }
    }
}

impl<G> Selector<G> for TournamentSelector<G> {
    #[allow(clippy::needless_range_loop)]
    fn select(&mut self, _population: &[G], costs: &[f64], partner_indices: &mut [usize]) {
        for lhs_index in 0..costs.len() {
            let mut rhs_index = 0;
            let mut rhs_cost = f64::MAX;
            for _ in 0..self.cross_over_candidates {
                let j = rand::thread_rng().gen_range(0..costs.len());
                let rhs_cost_candidate = *costs.get(j).unwrap();
                if rhs_cost_candidate < rhs_cost {
                    rhs_cost = rhs_cost_candidate;
                    rhs_index = j;
                }
            }
            partner_indices[lhs_index] = rhs_index;
        }
    }
}
