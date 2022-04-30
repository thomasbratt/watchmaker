use crate::solver::results::Results;
use crate::solver::stopping_criterion::StoppingCriterion;
use crate::Genetic;
use rand::Rng;
use std::fmt::Debug;
use std::time::{Duration, Instant};

pub type Progress<G> = Box<dyn FnMut(usize, Duration, f64, &G) -> bool>;

pub fn solve<G>(
    mut genetic: Box<dyn Genetic<G>>,
    mut progress: Option<Progress<G>>,
    cost_target: f64,
    epoch_limit: usize,
    mutation_rate: f64,
    population_size: usize,
    time_limit: Duration,
) -> Results<G>
where
    G: Clone + Debug + Eq + PartialEq,
{
    let start_time = Instant::now();

    let mut population = Vec::with_capacity(population_size);
    let mut replacement_population = Vec::with_capacity(population_size);
    let mut population_costs = Vec::with_capacity(population_size);

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
            population_costs.push(cost);
            if cost < best_cost {
                best_cost = cost;
                best_genome = genome.clone();
            }
        }

        let elapsed = Instant::now() - start_time;

        if progress.is_some()
            && !progress.as_mut().unwrap()(epoch, elapsed, best_cost, &best_genome)
        {
            return Results {
                reason: StoppingCriterion::StopRequested,
                epoch,
                elapsed,
                best_cost,
                mean_cost: mean_cost(&population_costs),
                worst_cost: worst_cost(&population_costs),
                best_genome,
            };
        }

        if epoch == epoch_limit {
            return Results {
                reason: StoppingCriterion::Epoch(epoch),
                epoch,
                elapsed,
                best_cost,
                mean_cost: mean_cost(&population_costs),
                worst_cost: worst_cost(&population_costs),
                best_genome,
            };
        }

        if best_cost <= cost_target {
            return Results {
                reason: StoppingCriterion::CostTargetReached(best_cost),
                epoch,
                elapsed,
                best_cost,
                mean_cost: mean_cost(&population_costs),
                worst_cost: worst_cost(&population_costs),
                best_genome,
            };
        }

        if elapsed >= time_limit {
            return Results {
                reason: StoppingCriterion::TimeOut(elapsed),
                epoch,
                elapsed,
                best_cost,
                mean_cost: mean_cost(&population_costs),
                worst_cost: worst_cost(&population_costs),
                best_genome,
            };
        }

        let candidate_count = 3;

        for lhs in population.iter() {
            let mut rhs_index = 0;
            let mut rhs_cost = f64::MAX;
            for _ in 0..candidate_count {
                let j = genetic.random().gen_range(0..population_costs.len());
                let rhs_cost_candidate = *population_costs.get(j).unwrap();
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

            replacement_population.push(m);
        }

        std::mem::swap(&mut population, &mut replacement_population);
        replacement_population.clear();
        population_costs.clear();
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
