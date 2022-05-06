use std::time::Duration;
use watchmaker::{Failure, SearchSettingsBuilder};

#[test]
fn fails_when_cross_over_candidates_too_low() {
    let result = TournamentSelector::new(0);

    assert_eq!(result.is_err(), true);
    assert_eq!(result.err().unwrap(), Failure::cross_over_candidates());
}
