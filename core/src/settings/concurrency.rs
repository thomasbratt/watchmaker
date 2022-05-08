use crate::settings::detect_concurrency::DetectConcurrencySettings;

/// Define the degree of parallelism to use when searching.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConcurrencySettings {
    /// Try both `MultiThreaded` and `SingleThreaded` and use the one that performs best.
    Detect(DetectConcurrencySettings),

    /// Use the concurrent, multithreaded variant of `select` that will use multiple cores.
    MultiThreaded,

    /// Use the single threaded variant of `select` that will use the current thread only.
    SingleThreaded,
}
