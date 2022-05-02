use crate::Failure;
use std::time::Duration;

// Define genetic search algorithm parameters.
#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    cost_target: f64,
    cross_over_candidates: usize,
    epoch_limit: usize,
    mutation_rate: f64,
    population_size: usize,
    time_limit: Duration,
}

impl Settings {
    pub fn cost_target(&self) -> f64 {
        self.cost_target
    }
    pub fn cross_over_candidates(&self) -> usize {
        self.cross_over_candidates
    }
    pub fn epoch_limit(&self) -> usize {
        self.epoch_limit
    }
    pub fn mutation_rate(&self) -> f64 {
        self.mutation_rate
    }
    pub fn population_size(&self) -> usize {
        self.population_size
    }
    pub fn time_limit(&self) -> Duration {
        self.time_limit
    }

    pub(super) fn new(
        cost_target: f64,
        cross_over_candidates: usize,
        epoch_limit: usize,
        mutation_rate: f64,
        population_size: usize,
        time_limit: Duration,
    ) -> Result<Settings, Failure> {
        if cross_over_candidates < 1 {
            return Err(Failure::cross_over_candidates());
        }

        if epoch_limit < 1 {
            return Err(Failure::epoch_limit());
        }

        if mutation_rate < 0.0 {
            return Err(Failure::mutation_rate());
        }

        if population_size < 1 {
            return Err(Failure::population_size());
        }

        if time_limit.is_zero() {
            return Err(Failure::time_limit());
        }

        Ok(Self {
            cost_target,
            cross_over_candidates,
            epoch_limit,
            mutation_rate,
            population_size,
            time_limit,
        })
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            cost_target: 0.0,
            cross_over_candidates: 16,
            epoch_limit: 1_024,
            mutation_rate: 0.01,
            population_size: 512,
            time_limit: Duration::from_secs(5),
        }
    }
}
