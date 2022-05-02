use std::time::Duration;

// Define the progress reporting callback.
pub type Progress<G> = Box<dyn FnMut(usize, Duration, f64, &G) -> bool>;
