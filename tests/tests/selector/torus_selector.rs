use std::collections::HashMap;
use tests::PeakGenome;
use tests::*;
use watchmaker::*;

#[test]
fn fails_when_cross_over_candidates_too_low() {
    let result: Result<TorusSelector<PeakGenome>, Failure> = TorusSelector::new(0, 10);

    assert_eq!(result.is_err(), true);
    assert_eq!(result.err().unwrap(), Failure::cross_over_candidates());
}

#[test]
fn fails_when_radius_too_low() {
    let result: Result<TorusSelector<PeakGenome>, Failure> = TorusSelector::new(10, 0);

    assert_eq!(result.is_err(), true);
    assert_eq!(result.err().unwrap(), Failure::cross_over_setting());
}

#[test]
fn when_costs_are_uniform_all_candidates_are_selected_evenly() {
    const ITERATIONS: usize = 4_096;
    const DELTA: usize = ITERATIONS / 10;

    let mut s: TorusSelector<u8> = TorusSelector::new(4, 8).unwrap();
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

    let mut s: TorusSelector<u8> = TorusSelector::new(4, 8).unwrap();
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

    let at_0 = *results.get(&0).unwrap();
    let at_7 = *results.get(&7).unwrap();
    let at_15 = *results.get(&15).unwrap();

    assert!(at_0 > at_7);
    assert!(at_7 > at_15);
}

#[test]
fn centre_of_torus_candidates_are_selected_within_radius() {
    const ITERATIONS: usize = 1_024;

    let mut s: TorusSelector<u8> = TorusSelector::new(2, 4).unwrap();
    let population = make_vec(16, || 0);
    let costs = make_vec(16, || 0.0);

    for _ in 0..ITERATIONS {
        let mut partner_indices = make_vec(16, || 0);
        s.select(&population, &costs, &mut partner_indices);
        for i in 4..12 {
            let difference = partner_indices[i].abs_diff(i);
            // eprintln!("M1:{}", difference);
            assert_between!(difference, 0, 4);
        }
    }
}

#[test]
fn start_of_torus_candidates_are_selected_within_radius() {
    const ITERATIONS: usize = 1_024;

    let mut s: TorusSelector<u8> = TorusSelector::new(2, 4).unwrap();
    let population = make_vec(16, || 0);
    let costs = make_vec(16, || 0.0);

    for _ in 0..ITERATIONS {
        let mut partner_indices = make_vec(16, || 0);
        s.select(&population, &costs, &mut partner_indices);
        for i in 0..4 {
            assert_not_between!(partner_indices[i], 8, 11);
        }
    }
}

#[test]
fn end_of_torus_candidates_are_selected_within_radius() {
    const ITERATIONS: usize = 1_024;

    let mut s: TorusSelector<u8> = TorusSelector::new(2, 4).unwrap();
    let population = make_vec(16, || 0);
    let costs = make_vec(16, || 0.0);

    for _ in 0..ITERATIONS {
        let mut partner_indices = make_vec(16, || 0);
        s.select(&population, &costs, &mut partner_indices);
        for i in 12..16 {
            assert_not_between!(partner_indices[i], 4, 7);
        }
    }
}
