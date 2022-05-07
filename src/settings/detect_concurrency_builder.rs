use crate::Failure;

use crate::settings::detect_concurrency::DetectConcurrencySettings;

/// Use to construct the settings required to execute a genetic algorithm search.
// #[derive(Clone, Debug, PartialEq)]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct DetectConcurrencySettingsBuilder {
    min_chunk_size: usize,
}

impl DetectConcurrencySettingsBuilder {
    /// The settings used to initialize the builder.
    pub fn from(settings: &DetectConcurrencySettings) -> Self {
        Self {
            min_chunk_size: settings.min_chunk_size(),
        }
    }

    /// The minimum population size to use in a thread.
    /// This is to prevent overhead and false sharing.
    pub fn min_chunk_size(&self) -> usize {
        self.min_chunk_size
    }

    /// Construct the settings required to execute a genetic algorithm search.
    pub fn build(&self) -> Result<DetectConcurrencySettings, Failure> {
        DetectConcurrencySettings::new(self.min_chunk_size)
    }
}

impl Default for DetectConcurrencySettingsBuilder {
    fn default() -> Self {
        DetectConcurrencySettingsBuilder::from(&DetectConcurrencySettings::default())
    }
}
