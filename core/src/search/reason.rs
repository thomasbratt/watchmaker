use std::time::Duration;

/// Define the reason the search terminated.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Reason {
    /// The search terminated as it reached the limit on the number of generations.
    Epoch(usize),

    /// The search terminated as it reached the timelimit.
    TimeOut(Duration),

    /// The search terminated as produced a genome that had a cost that was the same or equal to the
    /// cost target.
    CostTargetReached(f64),

    /// The search was requested to terminate.
    ///
    /// See [`crate::Progress`]
    StopRequested,
}
