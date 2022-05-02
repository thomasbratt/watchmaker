use std::time::Duration;

// Define the reasons the search terminated.
#[derive(Clone, Debug, PartialEq)]
pub enum Reason {
    Epoch(usize),
    TimeOut(Duration),
    CostTargetReached(f64),
    StopRequested,
}
