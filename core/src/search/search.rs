use crate::common::make_vec;
use crate::search::progress::ProgressSnapshot;
use crate::selector::Selector;
use crate::{
    largest, mean, ConcurrencySettings, Failure, Genetic, Progress, Reason, SearchSettings, Success,
};
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::fmt::Debug;
use std::time::Instant;

/// Search for a solution using a genetic algorithm.
/// This is the main entry point for the crate.
///
/// # Arguments
///
/// * `genetic` - Define the genetic operations on a chromosome `G`.
/// * `selector` - Define the algorithm used to select genome partners for cross over.
/// * `progress` - Define the progress reporting callback.
/// * `settings` - Configuration of genetic algorithm search.
///
pub fn search<G>(
    genetic: Box<dyn Genetic<G> + Send + Sync>,
    mut selector: Box<dyn Selector<G> + Send + Sync>,
    mut progress: Option<Progress<G>>,
    settings: &SearchSettings,
) -> Result<Success<G>, Failure>
where
    G: Clone + Debug + PartialEq + Send + Sync,
{
    let start_time = Instant::now();
    let mut population: Vec<G> = make_vec(settings.population_size(), || genetic.initialize());
    let mut replacement = population.clone();
    let mut partner_indices = make_vec(settings.population_size(), || 0_usize);
    let mut costs = make_vec(settings.population_size(), || 0.0);
    let mut epoch = 0;
    let mut best_cost = f64::MAX;
    let mut best_index = 0;
    let mut best_genome;

    loop {
        epoch += 1;

        match settings.concurrency() {
            ConcurrencySettings::MultiThreaded => {
                costs.par_iter_mut().enumerate().for_each(|(i, c)| {
                    *c = genetic.evaluate(population.get(i).unwrap());
                });
            }
            ConcurrencySettings::SingleThreaded | ConcurrencySettings::Detect(_) => {
                costs.iter_mut().enumerate().for_each(|(i, c)| {
                    *c = genetic.evaluate(population.get(i).unwrap());
                });
            }
        }

        selector.select(&population, &costs, &mut partner_indices);

        for ((i, lhs), rhs_index) in
            std::iter::zip(population.iter().enumerate(), partner_indices.iter())
        {
            let c = costs.get(i).unwrap();
            if *c < best_cost {
                best_cost = *c;
                best_index = i;
            }

            let rhs = population.get(*rhs_index).unwrap();

            let cross = if thread_rng().gen_bool(0.5) {
                genetic.crossover(lhs, rhs)
            } else {
                genetic.crossover(rhs, lhs)
            };

            let mutant = if thread_rng().gen_bool(settings.mutation_probability()) {
                genetic.mutate(&cross)
            } else {
                cross
            };

            replacement.push(mutant);
        }
        best_genome = &population[best_index];

        let elapsed = Instant::now() - start_time;

        if progress.is_some() {
            progress.as_mut().unwrap()(ProgressSnapshot::new(
                epoch,
                elapsed,
                best_cost,
                best_genome,
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
                best_genome.clone(),
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
                best_genome.clone(),
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
                best_genome.clone(),
            ));
        }

        std::mem::swap(&mut population, &mut replacement);
        replacement.clear();
    }
}
