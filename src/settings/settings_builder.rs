use crate::settings::settings::Settings;
use crate::Failure;

use std::time::Duration;

// A builder for the parameters of the genetic algorithm search.
#[derive(Clone, Debug, PartialEq)]
pub struct SettingsBuilder {
    cost_target: f64,
    cross_over_candidates: usize,
    epoch_limit: usize,
    mutation_rate: f64,
    population_size: usize,
    time_limit: Duration,
}

impl SettingsBuilder {
    pub fn from(settings: &Settings) -> Self {
        Self {
            cost_target: settings.cost_target(),
            cross_over_candidates: settings.cross_over_candidates(),
            epoch_limit: settings.epoch_limit(),
            mutation_rate: settings.mutation_rate(),
            population_size: settings.population_size(),
            time_limit: settings.time_limit(),
        }
    }

    pub fn cost_target(mut self, value: f64) -> Self {
        self.cost_target = value;
        self
    }

    pub fn cross_over_candidates(mut self, value: usize) -> Self {
        self.cross_over_candidates = value;
        self
    }

    pub fn epoch_limit(mut self, value: usize) -> Self {
        self.epoch_limit = value;
        self
    }

    pub fn mutation_rate(mut self, value: f64) -> Self {
        self.mutation_rate = value;
        self
    }

    pub fn population_size(mut self, value: usize) -> Self {
        self.population_size = value;
        self
    }

    pub fn time_limit(mut self, value: Duration) -> Self {
        self.time_limit = value;
        self
    }

    pub fn build(&self) -> Result<Settings, Failure> {
        Settings::new(
            self.cost_target,
            self.cross_over_candidates,
            self.epoch_limit,
            self.mutation_rate,
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
