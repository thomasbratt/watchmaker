use crate::selector::Selector;
use crate::Failure;
use rand::Rng;
use std::marker::PhantomData;

/// A diversity preserving population crossover selector.
/// Treats every genome in the population as having an (arbitrary) location on a 1 dimensional
/// torus.
/// For each genome in the population, select the best partner from N randomly chosen candidates
/// within a certain distance in both directions on the torus.
#[derive(Clone, Debug, PartialEq)]
pub struct TorusSelector<G> {
    cross_over_candidates: usize,
    radius: usize,
    _phantom_data: PhantomData<G>,
}

impl<G> TorusSelector<G> {
    /// Create new population selector.
    ///
    /// # Arguments
    ///
    /// * `cross_over_candidates` - The number of candidate genomes that will be compared when
    ///                             deciding which genome to use when new genomes are generated
    ///                             during cross over.
    /// * `radius` - The number of positions to search for candidates, in each direction.  
    pub fn new(cross_over_candidates: usize, radius: usize) -> Result<Self, Failure> {
        if cross_over_candidates < 1 {
            return Err(Failure::cross_over_candidates());
        }
        if radius < 1 {
            return Err(Failure::cross_over_setting());
        }
        Ok(Self {
            cross_over_candidates,
            radius,
            _phantom_data: Default::default(),
        })
    }

    /// The number of candidate genomes that will be compared when deciding which genome to
    /// use when new genomes are generated during cross over.  
    pub fn cross_over_candidates(&self) -> usize {
        self.cross_over_candidates
    }

    /// The number of positions to search for candidates, in each direction.
    pub fn radius(&self) -> usize {
        self.radius
    }
}

impl<G> Default for TorusSelector<G> {
    fn default() -> Self {
        Self {
            cross_over_candidates: 8,
            radius: 64,
            _phantom_data: Default::default(),
        }
    }
}

impl<G> Selector<G> for TorusSelector<G> {
    #[allow(clippy::needless_range_loop)]
    fn select(&mut self, _population: &[G], costs: &[f64], partner_indices: &mut [usize]) {
        for lhs_index in 0..costs.len() {
            let mut rhs_index = 0;
            let mut rhs_cost = f64::MAX;
            for _ in 0..self.cross_over_candidates {
                let q = lhs_index as isize
                    + rand::thread_rng()
                        .gen_range(-(self.radius as isize)..=(self.radius as isize));
                let j: usize = if q < 0 {
                    (q + costs.len() as isize) as usize
                } else if q >= costs.len() as isize {
                    (q - costs.len() as isize) as usize
                } else {
                    q as usize
                };
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
