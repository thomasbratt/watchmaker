use crate::search::progress::ProgressSnapshot;
use crate::selector::Selector;
use crate::{largest, mean, Failure, Genetic, Progress, Random, Reason, SearchSettings, Success};
use rand::Rng;
use std::fmt::Debug;
use std::iter::zip;
use std::time::Instant;

/// Search for a solution using a genetic algorithm.
/// This is the main entry point for the crate.
///
/// # Arguments
///
/// * `genetic` - Define the genetic operations on a chromosome `G`.
/// * `selector` - Define the algorithm used to select genome partners for cross over.
/// * `progress` - Define the progress reporting callback.
/// * `random` - Syntax sugar for a source of randomness chosen at runtime.
/// * `settings` - Configuration of genetic algorithm search.
///
pub fn search<G>(
    mut genetic: Box<dyn Genetic<G>>,
    mut selector: Box<dyn Selector<G>>,
    mut progress: Option<Progress<G>>,
    mut random: Random,
    settings: &SearchSettings,
) -> Result<Success<G>, Failure>
where
    G: Clone + Debug + PartialEq,
{
    let start_time = Instant::now();

    let mut population = Vec::with_capacity(settings.population_size());
    let mut replacement = population.clone();
    let mut partner_indices: Vec<usize> = (0..settings.population_size()).map(|_| 0).collect();
    let mut costs = Vec::with_capacity(settings.population_size());

    for _ in 0..settings.population_size() {
        population.push(genetic.initialize());
    }

    let mut epoch = 0;
    let mut best_cost = f64::MAX;
    let mut best_genome = population.get(0).unwrap().clone();

    loop {
        epoch += 1;

        for genome in &population {
            let cost = genetic.evaluate(genome);
            costs.push(cost);
            if cost < best_cost {
                best_cost = cost;
                best_genome = genome.clone();
            }
        }

        let elapsed = Instant::now() - start_time;

        if progress.is_some() {
            progress.as_mut().unwrap()(ProgressSnapshot::new(
                epoch,
                elapsed,
                best_cost,
                &best_genome,
            ));
        }

        if epoch == settings.epoch_limit() {
            return Ok(Success::new(
                Reason::Epoch(epoch),
                epoch,
                elapsed,
                best_cost,
                mean(&costs),
                largest(&costs),
                best_genome,
            ));
        }

        if best_cost <= settings.cost_target() {
            return Ok(Success::new(
                Reason::CostTargetReached(best_cost),
                epoch,
                elapsed,
                best_cost,
                mean(&costs),
                largest(&costs),
                best_genome,
            ));
        }

        if elapsed >= settings.time_limit() {
            return Ok(Success::new(
                Reason::TimeOut(elapsed),
                epoch,
                elapsed,
                best_cost,
                mean(&costs),
                largest(&costs),
                best_genome,
            ));
        }

        selector.select(&population, &costs, &mut random, &mut partner_indices);

        for (lhs, rhs_index) in zip(population.iter(), partner_indices.iter()) {
            let rhs = population.get(*rhs_index).unwrap();

            let cross = if random.gen_bool(0.5) {
                genetic.crossover(lhs, rhs)
            } else {
                genetic.crossover(rhs, lhs)
            };

            let mutant = if random.gen_bool(settings.mutation_probability()) {
                genetic.mutate(&cross)
            } else {
                cross
            };

            replacement.push(mutant);
        }

        std::mem::swap(&mut population, &mut replacement);
        replacement.clear();
        costs.clear();
    }
}
