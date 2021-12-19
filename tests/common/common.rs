pub fn check_equal_instruction(actual: Instruction, expected: Instruction) {
    if IS_LOGGING_ENABLED {
        println!("actual   : {:?}", actual);
        println!("expected : {:?}", expected);
    }
    assert_eq!(actual, expected);
}