use itertools::iproduct;
use rand::seq::SliceRandom;
use rand::Rng;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::time::Duration;
use watchmaker::*;

/// Show the effect of search hyper parameters on search times.
/// This example takes 00:02:46 (about 3 minutes) to run.
fn main() {
    let mut all: Vec<TrialResult> = make_trial_combinations(6..=16, 1..=8)
        .into_par_iter()
        .map(|trial| {
            let success = search(
                Box::new(TspGenetic::new(make_random())),
                Box::new(TournamentSelector::new(trial.cross_over_candidates).unwrap()),
                None,
                make_random(),
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

    all.sort_by(|lhs, rhs| {
        return if lhs.success.best_cost() < rhs.success.best_cost() {
            Ordering::Less
        } else {
            Ordering::Greater
        };
    });
    let top: Vec<TrialResult> = all.into_iter().take(10).collect();
    println!("Top {} hyperparameters", top.len());
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
    population: RangeInclusive<usize>,
    candidates: RangeInclusive<usize>,
) -> Vec<Trial> {
    iproduct!(
        population.map(|x| 2_usize.pow(x as u32)),
        candidates.map(|x| 2_usize.pow(x as u32))
    )
    .map(|(population_size, cross_over_candidates)| Trial {
        population_size,
        cross_over_candidates,
    })
    .filter(|x| x.cross_over_candidates <= x.population_size)
    .collect::<Vec<Trial>>()
}

pub struct Location {
    x: f64,
    y: f64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TspGenome {
    locations: Vec<usize>,
}

pub struct TspGenetic {
    random: Random,
    locations: Vec<Location>,
}

impl<'a> Genetic<TspGenome> for TspGenetic {
    fn initialize(&mut self) -> TspGenome {
        let mut locations: Vec<usize> = (0..self.locations.len()).collect();
        locations.shuffle(&mut self.random);
        TspGenome { locations }
    }

    fn evaluate(&mut self, genome: &TspGenome) -> f64 {
        let last = genome.locations.len() - 1;
        let mut cost = self.cost_of_arc(last, genome.locations[0]);
        for i in 0..last {
            cost += self.cost_of_arc(genome.locations[i], genome.locations[i + 1]);
        }
        cost
    }

    fn crossover(&mut self, lhs: &TspGenome, rhs: &TspGenome) -> TspGenome {
        let segment: usize = 10;
        let mut locations: Vec<usize> = lhs.locations.iter().take(segment).map(|x| *x).collect();
        let set_of_locations: HashSet<usize> = HashSet::from_iter(locations.iter().map(|x| *x));
        let mut b: Vec<usize> = rhs
            .locations
            .iter()
            .filter(|x| !set_of_locations.contains(x))
            .map(|x| *x)
            .collect();
        locations.append(&mut b);
        TspGenome { locations }
    }

    fn mutate(&mut self, genome: &TspGenome) -> TspGenome {
        let mut result = genome.clone();
        let lhs = self.random.gen_range(0..result.locations.len());
        let rhs = self.random.gen_range(0..result.locations.len());
        let temp = result.locations[lhs];
        result.locations[lhs] = result.locations[rhs];
        result.locations[rhs] = temp;
        result
    }
}

impl TspGenetic {
    fn cost_of_arc(&self, from: usize, to: usize) -> f64 {
        let lhs = &self.locations[from];
        let rhs = &self.locations[to];
        let x = (lhs.x - rhs.x) * (lhs.x - rhs.x);
        let y = (lhs.y - rhs.y) * (lhs.y - rhs.y);
        (x + y).sqrt()
    }

    pub fn new(random: Random) -> Self {
        // 52 Berlin location.
        // Best known tour that passes through each location exactly once has length of 7542.
        let locations: Vec<Location> = vec![
            Location { x: 565.0, y: 575.0 },
            Location { x: 25.0, y: 185.0 },
            Location { x: 345.0, y: 750.0 },
            Location { x: 945.0, y: 685.0 },
            Location { x: 845.0, y: 655.0 },
            Location { x: 880.0, y: 660.0 },
            Location { x: 25.0, y: 230.0 },
            Location {
                x: 525.0,
                y: 1000.0,
            },
            Location {
                x: 580.0,
                y: 1175.0,
            },
            Location {
                x: 650.0,
                y: 1130.0,
            },
            Location {
                x: 1605.0,
                y: 620.0,
            },
            Location {
                x: 1220.0,
                y: 580.0,
            },
            Location {
                x: 1465.0,
                y: 200.0,
            },
            Location { x: 1530.0, y: 5.0 },
            Location { x: 845.0, y: 680.0 },
            Location { x: 725.0, y: 370.0 },
            Location { x: 145.0, y: 665.0 },
            Location { x: 415.0, y: 635.0 },
            Location { x: 510.0, y: 875.0 },
            Location { x: 560.0, y: 365.0 },
            Location { x: 300.0, y: 465.0 },
            Location { x: 520.0, y: 585.0 },
            Location { x: 480.0, y: 415.0 },
            Location { x: 835.0, y: 625.0 },
            Location { x: 975.0, y: 580.0 },
            Location {
                x: 1215.0,
                y: 245.0,
            },
            Location {
                x: 1320.0,
                y: 315.0,
            },
            Location {
                x: 1250.0,
                y: 400.0,
            },
            Location { x: 660.0, y: 180.0 },
            Location { x: 410.0, y: 250.0 },
            Location { x: 420.0, y: 555.0 },
            Location { x: 575.0, y: 665.0 },
            Location {
                x: 1150.0,
                y: 1160.0,
            },
            Location { x: 700.0, y: 580.0 },
            Location { x: 685.0, y: 595.0 },
            Location { x: 685.0, y: 610.0 },
            Location { x: 770.0, y: 610.0 },
            Location { x: 795.0, y: 645.0 },
            Location { x: 720.0, y: 635.0 },
            Location { x: 760.0, y: 650.0 },
            Location { x: 475.0, y: 960.0 },
            Location { x: 95.0, y: 260.0 },
            Location { x: 875.0, y: 920.0 },
            Location { x: 700.0, y: 500.0 },
            Location { x: 555.0, y: 815.0 },
            Location { x: 830.0, y: 485.0 },
            Location { x: 1170.0, y: 65.0 },
            Location { x: 830.0, y: 610.0 },
            Location { x: 605.0, y: 625.0 },
            Location { x: 595.0, y: 360.0 },
            Location {
                x: 1340.0,
                y: 725.0,
            },
            Location {
                x: 1740.0,
                y: 245.0,
            },
        ];
        Self { random, locations }
    }
}
