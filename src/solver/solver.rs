use crate::solver::reason::Reason;
use crate::solver::success::Success;
use crate::{Failure, Genetic, Progress};
use rand::Rng;
use std::fmt::Debug;
use std::time::{Duration, Instant};

// Run a genetic algorithm search.
pub fn solve<G>(
    mut genetic: Box<dyn Genetic<G>>,
    mut progress: Option<Progress<G>>,
    cost_target: f64,
    cross_over_candidates: usize,
    epoch_limit: usize,
    mutation_rate: f64,
    population_size: usize,
    time_limit: Duration,
) -> Result<Success<G>, Failure>
where
    G: Clone + Debug + Eq + PartialEq,
{
    if cross_over_candidates < 1 {
        return Err(Failure::cross_over_candidates());
    }

    if epoch_limit < 1 {
        return Err(Failure::epoch_limit());
    }

    if mutation_rate < 0.0 {
        return Err(Failure::mutation_rate());
    }

    if population_size < 1 {
        return Err(Failure::population_size());
    }

    if time_limit.is_zero() {
        return Err(Failure::time_limit());
    }

    let start_time = Instant::now();

    let mut population = Vec::with_capacity(population_size);
    let mut replacement = Vec::with_capacity(population_size);
    let mut costs = Vec::with_capacity(population_size);

    for _ in 0..population_size {
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

        if progress.is_some()
            && !progress.as_mut().unwrap()(epoch, elapsed, best_cost, &best_genome)
        {
            return Ok(Success {
                reason: Reason::StopRequested,
                epoch,
                elapsed,
                best_cost,
                mean_cost: mean_cost(&costs),
                worst_cost: worst_cost(&costs),
                best_genome,
            });
        }

        if epoch == epoch_limit {
            return Ok(Success {
                reason: Reason::Epoch(epoch),
                epoch,
                elapsed,
                best_cost,
                mean_cost: mean_cost(&costs),
                worst_cost: worst_cost(&costs),
                best_genome,
            });
        }

        if best_cost <= cost_target {
            return Ok(Success {
                reason: Reason::CostTargetReached(best_cost),
                epoch,
                elapsed,
                best_cost,
                mean_cost: mean_cost(&costs),
                worst_cost: worst_cost(&costs),
                best_genome,
            });
        }

        if elapsed >= time_limit {
            return Ok(Success {
                reason: Reason::TimeOut(elapsed),
                epoch,
                elapsed,
                best_cost,
                mean_cost: mean_cost(&costs),
                worst_cost: worst_cost(&costs),
                best_genome,
            });
        }

        for lhs in population.iter() {
            let mut rhs_index = 0;
            let mut rhs_cost = f64::MAX;
            for _ in 0..cross_over_candidates {
                let j = genetic.random().gen_range(0..costs.len());
                let rhs_cost_candidate = *costs.get(j).unwrap();
                if rhs_cost_candidate < rhs_cost {
                    rhs_cost = rhs_cost_candidate;
                    rhs_index = j;
                }
            }
            let rhs = population.get(rhs_index).unwrap();
            let c = genetic.crossover(lhs, rhs);
            let m = if genetic.random().gen_bool(mutation_rate) {
                genetic.mutate(&c)
            } else {
                c.clone()
            };

            // if c != m {
            //     eprintln!("c:{:?}, m:{:?}", &c, &m);
            // }
            // eprintln!("lhs:{:?}, rhs:{:?}, c:{:?}, rhs_index:{}, rhs_cost:{}", lhs, rhs, c, rhs_index, rhs_cost);

            replacement.push(m);
        }

        std::mem::swap(&mut population, &mut replacement);
        replacement.clear();
        costs.clear();
    }
}

fn mean_cost(costs: &[f64]) -> f64 {
    costs.iter().fold(0.0, |acc, x| acc + x) / costs.len() as f64
}

fn worst_cost(costs: &[f64]) -> f64 {
    costs
        .iter()
        .fold(0.0, |acc, x| if *x > acc { *x } else { acc })
}
