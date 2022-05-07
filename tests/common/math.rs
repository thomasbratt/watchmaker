use watchmaker::{assert_between, round};

#[test]
fn can_round_to_zero_decimal_places() {
    let result = round(1234.5678, 10, 0);

    assert_eq!(result, 1234.0);
}

#[test]
fn can_round_to_1_decimal_places() {
    let result = round(1234.5678, 10, 0);

    assert_eq!(result, 1234.0);
}

#[test]
fn can_round_to_3_decimal_places() {
    let result = round(1234.5678, 10, 3);

    assert_between!(result, 1234.566, 1234.567);
}
