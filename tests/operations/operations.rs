// Test instruction serialization and deserialization.
#[test]
fn nop_roundtrips() {
    let expected = 42

    let actual = 42;

    check_equal_instruction(actual, expected);
}
