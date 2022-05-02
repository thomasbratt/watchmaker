// Define failure conditions for crate.
#[derive(Clone, Debug, PartialEq)]
pub struct Failure {
    error: i32,
    message: Box<str>,
}

impl Failure {
    pub fn cross_over_candidates() -> Failure {
        Failure {
            error: 1,
            message: Box::from("number of cross over candidates must be at least 1"),
        }
    }

    pub fn epoch_limit() -> Self {
        Failure {
            error: 2,
            message: Box::from("epoch limit must be at least 1"),
        }
    }

    pub fn mutation_rate() -> Self {
        Failure {
            error: 3,
            message: Box::from("mutation rate must be positive"),
        }
    }

    pub fn population_size() -> Self {
        Failure {
            error: 4,
            message: Box::from("population size must be at least 1"),
        }
    }

    pub fn time_limit() -> Self {
        Failure {
            error: 5,
            message: Box::from("time limit must be non-zero"),
        }
    }
}
