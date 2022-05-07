use rayon::prelude::*;
use std::ops::RangeInclusive;
use std::time::Duration;
use watchmaker::*;

/// Show the effect of search hyper parameters on search times.
/// This example takes 00:02:46 (about 3 minutes) to run.
fn main() {
    let results: Vec<TrialResult> = make_trial_combinations(&(6..=16), &(1..=8))
        .into_par_iter()
        .map(|trial| {
            let success = search(
                Box::new(TspGenetic::default()),
                Box::new(TournamentSelector::new(trial.cross_over_candidates).unwrap()),
                None,
                &SearchSettingsBuilder::default()
                    .population_size(trial.population_size)
                    .mutation_probability(0.01)
                    .time_limit(Duration::from_secs(15))
                    .epoch_limit(1_000_000)
                    .build()
                    .unwrap(),
            )
            .unwrap();
            println!(
            "(population_size:{:10}, cross_over_candidates:{:10}) -> (cost:{:10.2}, elapsed:{:?}) ",
            trial.population_size,
            trial.cross_over_candidates,
            success.best_cost(),
            success.elapsed()
        );
            TrialResult { success, trial }
        })
        .collect();

    display_top_n(&results, 10);
}

fn display_top_n(results: &[TrialResult], num_rows: usize) {
    let mut all = Vec::from(results);
    all.sort_by(|lhs, rhs| {
        lhs.success
            .best_cost()
            .partial_cmp(&rhs.success.best_cost())
            .unwrap()
    });
    let top: Vec<TrialResult> = all.into_iter().take(num_rows).collect();
    println!("Top {} hyper parameters", top.len());
    for item in &top {
        println!(
            "population:{:10}, candidates:{:10} -> best_cost:{:10.2},mean_cost:{:10.2},{:?}",
            item.trial.population_size,
            item.trial.cross_over_candidates,
            item.success.best_cost(),
            item.success.mean_cost(),
            item.success.reason()
        );
    }
    for (i, item) in top.iter().enumerate() {
        println!("{:6}: {:?}", i, item.success.best_genome());
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Trial {
    population_size: usize,
    cross_over_candidates: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TrialResult {
    success: Success<TspGenome>,
    trial: Trial,
}

fn make_trial_combinations(
    population: &RangeInclusive<usize>,
    candidates: &RangeInclusive<usize>,
) -> Vec<Trial> {
    let mut results = Vec::new();
    for population_size in population.clone().map(|x| 2_usize.pow(x as u32)) {
        for cross_over_candidates in candidates.clone().map(|x| 2_usize.pow(x as u32)) {
            if cross_over_candidates > population_size {
                continue;
            }
            results.push(Trial {
                population_size,
                cross_over_candidates,
            })
        }
    }
    results
}
