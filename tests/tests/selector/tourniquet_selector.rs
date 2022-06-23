use std::collections::HashMap;
use tests::assert_between;
use tests::PeakGenome;
use watchmaker::*;

#[test]
fn fails_when_cross_over_candidates_too_low() {
    let result: Result<TourniquetSelector<PeakGenome>, Failure> = TourniquetSelector::new(0);

    assert_eq!(result.is_err(), true);
    assert_eq!(result.err().unwrap(), Failure::cross_over_candidates());
}

#[test]
fn when_costs_are_uniform_all_candidates_are_selected_evenly() {
    const ITERATIONS: usize = 4_096;
    const DELTA: usize = ITERATIONS / 10;

    let mut s: TourniquetSelector<u8> = TourniquetSelector::new(4).unwrap();
    let population = make_vec(16, || 0);
    let costs = make_vec(16, || 0.0);

    let mut results: HashMap<usize, usize> = HashMap::new();
    for _ in 0..ITERATIONS {
        let mut partner_indices = make_vec(16, || 0);
        s.select(&population, &costs, &mut partner_indices);
        for i in &partner_indices {
            *results.entry(*i).or_insert(0) += 1;
        }
    }

    for count in results.values() {
        assert_between!(*count, ITERATIONS - DELTA, ITERATIONS + DELTA);
    }
}

#[test]
fn when_costs_are_not_uniform_candidates_are_selected_according_to_distribution() {
    const ITERATIONS: usize = 4_096;

    let mut s: TourniquetSelector<u8> = TourniquetSelector::new(4).unwrap();
    let population = make_vec(16, || 0);
    let costs = (0..16).map(|x| x as f64).collect::<Vec<f64>>();

    let mut results: HashMap<usize, usize> = HashMap::new();
    for i in 0..16 {
        results.insert(i, 0);
    }
    for _ in 0..ITERATIONS {
        let mut partner_indices = make_vec(16, || 0);
        s.select(&population, &costs, &mut partner_indices);
        for i in &partner_indices {
            results.entry(*i).and_modify(|e| *e += 1);
        }
    }

    // for i in 0..16_usize {
    //     println!("{:2}, {:6}", i, results.get(&i).unwrap());
    // }

    let at_0 = *results.get(&0).unwrap();
    let at_7 = *results.get(&7).unwrap();
    let at_15 = *results.get(&15).unwrap();

    assert!(at_0 > at_7);
    assert!(at_7 > at_15);
}
