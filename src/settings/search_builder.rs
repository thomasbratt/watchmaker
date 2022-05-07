use crate::settings::concurrency::ConcurrencySettings;
use crate::settings::search::SearchSettings;
use crate::Failure;
use std::time::Duration;

/// Use to construct the settings required to execute a genetic algorithm search.
// #[derive(Clone, Debug, PartialEq)]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct SearchSettingsBuilder {
    concurrency: ConcurrencySettings,
    cost_target: f64,
    epoch_limit: usize,
    mutation_probability: f64,
    population_size: usize,
    time_limit: Duration,
}

impl SearchSettingsBuilder {
    /// The settings used to initialize the builder.
    pub fn from(settings: &SearchSettings) -> Self {
        Self {
            concurrency: settings.concurrency(),
            cost_target: settings.cost_target(),
            epoch_limit: settings.epoch_limit(),
            mutation_probability: settings.mutation_probability(),
            population_size: settings.population_size(),
            time_limit: settings.time_limit(),
        }
    }

    //// Define the degree of parallelism to use when searching.
    pub fn concurrency(mut self, value: ConcurrencySettings) -> Self {
        self.concurrency = value;
        self
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
    pub fn build(&self) -> Result<SearchSettings, Failure> {
        SearchSettings::new(
            self.concurrency,
            self.cost_target,
            self.epoch_limit,
            self.mutation_probability,
            self.population_size,
            self.time_limit,
        )
    }
}

impl Default for SearchSettingsBuilder {
    fn default() -> Self {
        SearchSettingsBuilder::from(&SearchSettings::default())
    }
}
