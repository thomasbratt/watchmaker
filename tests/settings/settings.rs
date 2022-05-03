use std::time::Duration;
use watchmaker::{Failure, SettingsBuilder};

#[test]
fn fails_when_cross_over_candidates_too_low() {
    let result = SettingsBuilder::default().cross_over_candidates(0).build();

    assert_eq!(result.is_err(), true);
    assert_eq!(result.err().unwrap(), Failure::cross_over_candidates());
}

#[test]
fn fails_when_epoch_limit_too_low() {
    let result = SettingsBuilder::default().epoch_limit(0).build();

    assert_eq!(result.is_err(), true);
    assert_eq!(result.err().unwrap(), Failure::epoch_limit());
}

#[test]
fn fails_when_mutation_rate_is_negative() {
    let result = SettingsBuilder::default()
        .mutation_probability(-1.0)
        .build();

    assert_eq!(result.is_err(), true);
    assert_eq!(result.err().unwrap(), Failure::mutation_probability());
}

#[test]
fn fails_when_population_size_is_too_small() {
    let result = SettingsBuilder::default().population_size(0).build();

    assert_eq!(result.is_err(), true);
    assert_eq!(result.err().unwrap(), Failure::population_size());
}

#[test]
fn fails_when_time_limit_is_zero() {
    let result = SettingsBuilder::default()
        .time_limit(Duration::ZERO)
        .build();

    assert_eq!(result.is_err(), true);
    assert_eq!(result.err().unwrap(), Failure::time_limit());
}
