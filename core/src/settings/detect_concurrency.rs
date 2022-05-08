use crate::Failure;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DetectConcurrencySettings {
    min_chunk_size: usize,
}

impl DetectConcurrencySettings {
    /// The minimum population size to use in a thread.
    /// This is to prevent overhead and false sharing.
    pub fn min_chunk_size(&self) -> usize {
        self.min_chunk_size
    }

    pub(super) fn new(min_chunk_size: usize) -> Result<DetectConcurrencySettings, Failure> {
        if min_chunk_size < 1 {
            return Err(Failure::min_chunk_size());
        }

        Ok(Self { min_chunk_size })
    }
}

impl Default for DetectConcurrencySettings {
    /// The default search settings.
    /// These are suitable for small search problems.
    fn default() -> Self {
        Self { min_chunk_size: 1 }
    }
}
