use crate::settings::settings::Settings;
use crate::Failure;

use std::time::Duration;

/// Use to construct the settings required to execute a genetic algorithm search.
#[derive(Clone, Debug, PartialEq)]
pub struct SettingsBuilder {
    cost_target: f64,
    cross_over_candidates: usize,
    epoch_limit: usize,
    mutation_probability: f64,
    population_size: usize,
    time_limit: Duration,
}

impl SettingsBuilder {
    /// The settings used to initialize the builder.
    pub fn from(settings: &Settings) -> Self {
        Self {
            cost_target: settings.cost_target(),
            cross_over_candidates: settings.cross_over_candidates(),
            epoch_limit: settings.epoch_limit(),
            mutation_probability: settings.mutation_probability(),
            population_size: settings.population_size(),
            time_limit: settings.time_limit(),
        }
    }

    /// The genome cost that will terminate the search.
    /// Genome costs are expected to be higher at the start of the search process and become
    /// smaller as the search progresses.
    /// The search will terminate when the cost of any genome in the population is less than or
    /// equal to this value.   
    pub fn cost_target(mut self, value: f64) -> Self {
        self.cost_target = value;
        self
    }

    /// The number of candidate genomes that will be compared when deciding which genome to
    /// use when new genomes are generated during cross over.  
    pub fn cross_over_candidates(mut self, value: usize) -> Self {
        self.cross_over_candidates = value;
        self
    }

    /// The number of maximum number of epochs (iterations) that the search should execute
    /// before terminating the search.
    pub fn epoch_limit(mut self, value: usize) -> Self {
        self.epoch_limit = value;
        self
    }

    /// The probability of a newly generated genome being mutated.
    /// The mutation implementation is defined by the [`mutate()`](crate::Genetic#mutate()).
    pub fn mutation_probability(mut self, value: f64) -> Self {
        self.mutation_probability = value;
        self
    }

    /// The number of genomes that exist at any one epoch (iteration) of the search.
    pub fn population_size(mut self, value: usize) -> Self {
        self.population_size = value;
        self
    }

    /// The maximum duration of the search.
    pub fn time_limit(mut self, value: Duration) -> Self {
        self.time_limit = value;
        self
    }

    /// Construct the settings required to execute a genetic algorithm search.
    pub fn build(&self) -> Result<Settings, Failure> {
        Settings::new(
            self.cost_target,
            self.cross_over_candidates,
            self.epoch_limit,
            self.mutation_probability,
            self.population_size,
            self.time_limit,
        )
    }
}

impl Default for SettingsBuilder {
    fn default() -> Self {
        SettingsBuilder::from(&Settings::default())
    }
}
