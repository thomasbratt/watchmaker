use std::time::Duration;

/// Define the progress reporting callback.
///
/// Implementors should return true to continue the search and false to terminate it.
pub type Progress<G> = Box<dyn FnMut(usize, Duration, f64, &G) -> bool>;
