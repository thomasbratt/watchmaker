use watchmaker::{make_random, search, Settings, WSGenetic, TARGET};

#[test]
fn search_finds_result_for_simple_test_case() {
    let result = search(
        Box::new(WSGenetic::new(make_random())),
        None,
        make_random(),
        &Settings::default(),
    );

    eprintln!("{:?}", result);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.as_ref().ok().unwrap().best_cost(), 0.0);
    assert_eq!(result.as_ref().ok().unwrap().best_genome().0, TARGET);
}
