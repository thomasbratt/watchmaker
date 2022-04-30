use crate::solver::results::Results;
use crate::solver::stopping_criterion::StoppingCriterion;
use crate::Genetic;
use rand::Rng;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::time::{Duration, Instant};

type Progress<G> = Box<dyn FnMut(usize, Duration, f64, &G) -> bool>;

pub struct Solver<G>
where
    G: Clone + Debug + Eq + PartialEq,
{
    phantom_data: PhantomData<G>,
    genetic: Box<dyn Genetic<G>>,
    mutation_rate: f64,
    population_size: usize,
    epoch_limit: usize,
    time_limit: Duration,
    cost_target: f64,
    progress: Option<Progress<G>>,
}

impl<G> Solver<G>
where
    G: Clone + Debug + Eq + PartialEq,
{
    pub fn new(
        genetic: Box<dyn Genetic<G>>,
        mutation_rate: f64,
        population_size: usize,
        epoch_limit: usize,
        time_limit: Duration,
        cost_target: f64,
        progress: Option<Progress<G>>,
    ) -> Self {
        Self {
            phantom_data: Default::default(),
            genetic,
            mutation_rate,
            population_size,
            epoch_limit,
            time_limit,
            cost_target,
            progress,
        }
    }

    pub fn solve(&mut self) -> Results<G> {
        let start_time = Instant::now();

        let mut population = Vec::with_capacity(self.population_size);
        let mut replacement_population = Vec::with_capacity(self.population_size);
        let mut population_costs = Vec::with_capacity(self.population_size);

        for _ in 0..self.population_size {
            population.push(self.genetic.initialize());
        }

        let mut epoch = 0;
        let mut best_cost = f64::MAX;
        let mut best_genome = population.get(0).unwrap().clone();

        loop {
            epoch += 1;

            for genome in &population {
                let cost = self.genetic.evaluate(genome);
                population_costs.push(cost);
                if cost < best_cost {
                    best_cost = cost;
                    best_genome = genome.clone();
                }
            }

            let elapsed = Instant::now() - start_time;

            if self.progress.is_some()
                && !self.progress.as_mut().unwrap()(epoch, elapsed, best_cost, &best_genome)
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

            if epoch == self.epoch_limit {
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

            if best_cost <= self.cost_target {
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

            if elapsed >= self.time_limit {
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
                    let j = self.genetic.random().gen_range(0..population_costs.len());
                    let rhs_cost_candidate = *population_costs.get(j).unwrap();
                    if rhs_cost_candidate < rhs_cost {
                        rhs_cost = rhs_cost_candidate;
                        rhs_index = j;
                    }
                }
                let rhs = population.get(rhs_index).unwrap();
                let c = self.genetic.crossover(lhs, rhs);
                let m = if self.genetic.random().gen_bool(self.mutation_rate) {
                    self.genetic.mutate(&c)
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
}

fn mean_cost(costs: &[f64]) -> f64 {
    costs.iter().fold(0.0, |acc, x| acc + x) / costs.len() as f64
}

fn worst_cost(costs: &[f64]) -> f64 {
    costs
        .iter()
        .fold(0.0, |acc, x| if *x > acc { *x } else { acc })
}
