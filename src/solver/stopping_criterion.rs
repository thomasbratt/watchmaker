use std::time::Duration;

// Defines when to stop the search.
pub enum StoppingCriterion {
    Epoch(usize),
    TimeOut(Duration),
    CostTargetReached(f64),
}
