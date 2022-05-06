use crate::Failure;
use std::time::Duration;

/// The settings for a genetic algorithm search.
#[derive(Clone, Debug, PartialEq)]
pub struct SearchSettings {
    cost_target: f64,
    epoch_limit: usize,
    mutation_probability: f64,
    population_size: usize,
    time_limit: Duration,
}

impl SearchSettings {
    /// The genome cost that will terminate the search.
    /// Genome costs are expected to be higher at the start of the search process and become
    /// smaller as the search progresses.
    /// The search will terminate when the cost of any genome in the population is less than or
    /// equal to this value.   
    pub fn cost_target(&self) -> f64 {
        self.cost_target
    }

    /// The number of maximum number of epochs (iterations) that the search should execute
    /// before terminating the search.
    pub fn epoch_limit(&self) -> usize {
        self.epoch_limit
    }

    /// The probability of a newly generated genome being mutated.
    /// The mutation implementation is defined by the [`mutate()`](crate::Genetic#mutate()).
    pub fn mutation_probability(&self) -> f64 {
        self.mutation_probability
    }

    /// The number of genomes that exist at any one epoch (iteration) of the search.
    pub fn population_size(&self) -> usize {
        self.population_size
    }

    /// The maximum duration of the search.
    pub fn time_limit(&self) -> Duration {
        self.time_limit
    }

    pub(super) fn new(
        cost_target: f64,
        epoch_limit: usize,
        mutation_rate: f64,
        population_size: usize,
        time_limit: Duration,
    ) -> Result<SearchSettings, Failure> {
        if epoch_limit < 1 {
            return Err(Failure::epoch_limit());
        }

        if mutation_rate < 0.0 {
            return Err(Failure::mutation_probability());
        }

        if population_size < 1 {
            return Err(Failure::population_size());
        }

        if time_limit.is_zero() {
            return Err(Failure::time_limit());
        }

        Ok(Self {
            cost_target,
            epoch_limit,
            mutation_probability: mutation_rate,
            population_size,
            time_limit,
        })
    }
}

impl Default for SearchSettings {
    /// The default search settings.
    /// These will only be suitable for very small search problems.
    fn default() -> Self {
        SearchSettings {
            cost_target: 0.0,
            epoch_limit: 1_024,
            mutation_probability: 0.01,
            population_size: 1_024,
            time_limit: Duration::from_secs(5),
        }
    }
}
