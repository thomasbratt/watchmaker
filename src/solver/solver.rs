use crate::common::Random;
use crate::solver::results::Results;
use crate::solver::stopping_criterion::StoppingCriterion;
use crate::Genetic;
use rand::Rng;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::time::{Duration, Instant};

type Progress = dyn FnMut(usize, Duration, f64, &G) -> bool;

pub struct Solver<G>
where
    G: Genetic<G>,
{
    phantom_data: PhantomData<G>,
    random: Random,
    mutation_rate: f64,
    population_size: usize,
    epoch_limit: usize,
    time_limit: Duration,
    cost_target: f64,
    progress: Option<Progress>,
}

impl<G> Solver<G>
where
    G: Debug + Genetic<G>,
{
    pub fn new(
        random: Random,
        mutation_rate: f64,
        population_size: usize,
        epoch_limit: usize,
        time_limit: Duration,
        cost_target: f64,
        progress: Option<Progress>,
    ) -> Self {
        Self {
            phantom_data: Default::default(),
            random,
            mutation_rate,
            population_size,
            epoch_limit,
            time_limit,
            cost_target,
            progress,
        }
    }

    pub fn solve(mut self) -> Results<G> {
        let start_time = Instant::now();

        let mut population = Vec::with_capacity(self.population_size);
        for _ in 0..self.population_size {
            population.push(G::initialize(&mut self.random));
        }

        let mut epoch = 1;
        let mut best_cost = f64::max_value();
        let mut best_genome = population.get(0).unwrap().clone();

        loop {
            let mut population_costs = Vec::with_capacity(self.population_size);
            for genome in &population {
                let cost = G::evaluate(&genome);
                population_costs.push(cost);
                if cost < best_cost {
                    best_cost = cost;
                    best_genome = genome.clone();
                }
            }

            let elapsed = Instant::now() - start_time;

            if let Some(p) = &self.progress {
                p(epoch, elapsed, best_cost, best_genome);
            }

            if epoch == self.epoch_limit {
                return Results {
                    reason: StoppingCriterion::Epoch(epoch),
                    epoch: 0,
                    elapsed,
                    best_cost: 0.0,
                    mean_cost: 0.0,
                    worst_cost: 0.0,
                    best_genome,
                };
            }

            if best_cost <= self.cost_target {
                return Results {
                    reason: StoppingCriterion::CostTargetReached(best_cost),
                    epoch,
                    elapsed,
                    best_cost,
                    mean_cost: 0.0,
                    worst_cost: 0.0,
                    best_genome,
                };
            }

            if elapsed >= self.time_limit {
                return Results {
                    reason: StoppingCriterion::TimeOut(elapsed),
                    epoch,
                    elapsed,
                    best_cost,
                    mean_cost: 0.0,
                    worst_cost: 0.0,
                    best_genome,
                };
            }

            // Crossover

            // let x: u16 = self.random.gen();
            // println!("random: {0}", x);

            // Mutate
        }
    }
}
