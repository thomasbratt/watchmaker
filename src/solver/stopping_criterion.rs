use std::time::Duration;

// Defines when to stop the search.
pub enum StoppingCriterion {
    EpochCount(u64),
    TimeLimit(Duration),
    CostLessThanOrEqualTo(f64),
}
