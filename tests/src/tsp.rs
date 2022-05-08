use rand::prelude::SliceRandom;
use rand::Rng;
use std::collections::HashSet;
use watchmaker::*;

pub struct Location {
    x: f64,
    y: f64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TspGenome {
    locations: Vec<usize>,
}

pub struct TspGenetic {
    locations: Vec<Location>,
}

impl<'a> Genetic<TspGenome> for TspGenetic {
    fn initialize(&self) -> TspGenome {
        let mut locations: Vec<usize> = (0..self.locations.len()).collect();
        locations.shuffle(&mut rand::thread_rng());
        TspGenome { locations }
    }

    fn evaluate(&self, genome: &TspGenome) -> f64 {
        let last = genome.locations.len() - 1;
        let mut cost = self.cost_of_arc(last, genome.locations[0]);
        for i in 0..last {
            cost += self.cost_of_arc(genome.locations[i], genome.locations[i + 1]);
        }
        cost
    }

    fn crossover(&self, lhs: &TspGenome, rhs: &TspGenome) -> TspGenome {
        let segment: usize = 10;
        let mut locations: Vec<usize> = lhs.locations.iter().take(segment).copied().collect();
        let set_of_locations: HashSet<usize> = HashSet::from_iter(locations.iter().copied());
        let mut b: Vec<usize> = rhs
            .locations
            .iter()
            .filter(|x| !set_of_locations.contains(x))
            .copied()
            .collect();
        locations.append(&mut b);
        TspGenome { locations }
    }

    fn mutate(&self, genome: &TspGenome) -> TspGenome {
        let mut result = genome.clone();
        let lhs = rand::thread_rng().gen_range(0..result.locations.len());
        let rhs = rand::thread_rng().gen_range(0..result.locations.len());
        result.locations.swap(lhs, rhs);
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
}

impl Default for TspGenetic {
    fn default() -> Self {
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
        Self { locations }
    }
}
