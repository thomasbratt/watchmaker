/// Define failure conditions for the entire crate.
#[derive(Clone, Debug, PartialEq, Hash)]
pub struct Failure {
    error: i32,
    message: Box<str>,
}

impl Failure {
    /// Return a POSIX exit code for the failure case.
    pub fn error(&self) -> i32 {
        self.error
    }

    /// Return a human readable error message as a dynamically allocated immutable str.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Return a failure for the case when the number of cross over candidates is invalid.
    pub fn cross_over_candidates() -> Self {
        Failure {
            error: 1,
            message: Box::from("number of cross over candidates must be at least 1"),
        }
    }

    /// Return a failure for the case when the epoch limit is invalid.
    pub fn epoch_limit() -> Self {
        Failure {
            error: 2,
            message: Box::from("epoch limit must be at least 1"),
        }
    }

    /// Return a failure for the case when the mutation probability is invalid.
    pub fn mutation_probability() -> Self {
        Failure {
            error: 3,
            message: Box::from("mutation probability must be in the range [0..1]"),
        }
    }

    /// Return a failure for the case when the population size is invalid.
    pub fn population_size() -> Self {
        Failure {
            error: 4,
            message: Box::from("population size must be at least 1"),
        }
    }

    /// Return a failure for an invalid multithreading setting `min_chunk_size`.
    pub fn min_chunk_size() -> Self {
        Failure {
            error: 5,
            message: Box::from("min_chunk_size size must be at least 1"),
        }
    }

    /// Return a failure for the case when the time limit is invalid.
    pub fn time_limit() -> Self {
        Failure {
            error: 6,
            message: Box::from("time limit must be non-zero"),
        }
    }
}
