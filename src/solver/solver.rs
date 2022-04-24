use crate::common::Random;
use crate::solver::results::Results;
use crate::solver::stopping_criterion::StoppingCriterion;
use crate::Genetic;
use rand::Rng;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::time::Duration;

pub struct Solver<G>
where
    G: Genetic<G>,
{
    phantom_data: PhantomData<G>,
    random: Random,
    mutation_rate: f64,
    population_size: usize,
    epoch_count: usize,
    time_limit: Duration,
    cost_target: f64,
}

impl<G> Solver<G>
where
    G: Debug + Genetic<G>,
{
    pub fn new(
        random: Random,
        mutation_rate: f64,
        population_size: usize,
        epoch_count: usize,
        time_limit: Duration,
        cost_target: f64,
    ) -> Self {
        Self {
            phantom_data: Default::default(),
            random,
            mutation_rate,
            population_size,
            epoch_count,
            time_limit,
            cost_target,
        }
    }

    pub fn solve(mut self) -> Results<G> {
        let x: u16 = self.random.gen();
        println!("random: {0}", x);

        let mut population = Vec::with_capacity(self.population_size);
        for _ in 0..self.population_size {
            population.push(G::initialize(&mut self.random));
        }

        let best_genome = G::initialize(&mut self.random);

        println!("best_genome: {0:?}", best_genome);

        Results {
            reason: StoppingCriterion::EpochCount(2123),
            epoch_count: 0,
            duration: Default::default(),
            best_cost: 0.0,
            mean_cost: 0.0,
            worst_cost: 0.0,
            best_genome,
        }
    }
}
