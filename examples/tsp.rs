use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashSet;
use std::time::Duration;
use watchmaker::*;

// Search for a solution to a well-known TSP problem.
fn main() {
    let result = search(
        Box::new(TspGenetic::new(make_random())),
        Box::new(TournamentSelector::new(8).unwrap()),
        Some(Box::new(|snapshot| {
            if snapshot.epoch() % 10 == 1 {
                println!("progress snapshot:{:?}", snapshot);
            }
        })),
        make_random(),
        &SearchSettingsBuilder::default()
            .population_size(10_000)
            .mutation_probability(0.01)
            .time_limit(Duration::from_secs(15))
            .epoch_limit(1_000_000)
            .build()
            .unwrap(),
    )
    .unwrap();
    println!("{:?}", result);
}

#[derive(Clone, Debug, PartialEq)]
pub struct Location<'a> {
    name: Box<&'a str>,
    x: f64,
    y: f64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TspGenome {
    locations: Vec<usize>,
}

pub struct TspGenetic<'a> {
    random: Random,
    locations: Vec<Location<'a>>,
}

impl<'a> TspGenetic<'a> {
    pub fn new(random: Random) -> Self {
        // 52 Berlin location.
        // Best known tour that passes through each location exactly once has length of 7542.
        let locations: Vec<Location> = vec![
            Location {
                name: Box::new("1"),
                x: 565.0,
                y: 575.0,
            },
            Location {
                name: Box::new("2"),
                x: 25.0,
                y: 185.0,
            },
            Location {
                name: Box::new("3"),
                x: 345.0,
                y: 750.0,
            },
            Location {
                name: Box::new("4"),
                x: 945.0,
                y: 685.0,
            },
            Location {
                name: Box::new("5"),
                x: 845.0,
                y: 655.0,
            },
            Location {
                name: Box::new("6"),
                x: 880.0,
                y: 660.0,
            },
            Location {
                name: Box::new("7"),
                x: 25.0,
                y: 230.0,
            },
            Location {
                name: Box::new("8"),
                x: 525.0,
                y: 1000.0,
            },
            Location {
                name: Box::new("9"),
                x: 580.0,
                y: 1175.0,
            },
            Location {
                name: Box::new("10"),
                x: 650.0,
                y: 1130.0,
            },
            Location {
                name: Box::new("11"),
                x: 1605.0,
                y: 620.0,
            },
            Location {
                name: Box::new("12"),
                x: 1220.0,
                y: 580.0,
            },
            Location {
                name: Box::new("13"),
                x: 1465.0,
                y: 200.0,
            },
            Location {
                name: Box::new("14"),
                x: 1530.0,
                y: 5.0,
            },
            Location {
                name: Box::new("15"),
                x: 845.0,
                y: 680.0,
            },
            Location {
                name: Box::new("16"),
                x: 725.0,
                y: 370.0,
            },
            Location {
                name: Box::new("17"),
                x: 145.0,
                y: 665.0,
            },
            Location {
                name: Box::new("18"),
                x: 415.0,
                y: 635.0,
            },
            Location {
                name: Box::new("19"),
                x: 510.0,
                y: 875.0,
            },
            Location {
                name: Box::new("20"),
                x: 560.0,
                y: 365.0,
            },
            Location {
                name: Box::new("21"),
                x: 300.0,
                y: 465.0,
            },
            Location {
                name: Box::new("22"),
                x: 520.0,
                y: 585.0,
            },
            Location {
                name: Box::new("23"),
                x: 480.0,
                y: 415.0,
            },
            Location {
                name: Box::new("24"),
                x: 835.0,
                y: 625.0,
            },
            Location {
                name: Box::new("25"),
                x: 975.0,
                y: 580.0,
            },
            Location {
                name: Box::new("26"),
                x: 1215.0,
                y: 245.0,
            },
            Location {
                name: Box::new("27"),
                x: 1320.0,
                y: 315.0,
            },
            Location {
                name: Box::new("28"),
                x: 1250.0,
                y: 400.0,
            },
            Location {
                name: Box::new("29"),
                x: 660.0,
                y: 180.0,
            },
            Location {
                name: Box::new("30"),
                x: 410.0,
                y: 250.0,
            },
            Location {
                name: Box::new("31"),
                x: 420.0,
                y: 555.0,
            },
            Location {
                name: Box::new("32"),
                x: 575.0,
                y: 665.0,
            },
            Location {
                name: Box::new("33"),
                x: 1150.0,
                y: 1160.0,
            },
            Location {
                name: Box::new("34"),
                x: 700.0,
                y: 580.0,
            },
            Location {
                name: Box::new("35"),
                x: 685.0,
                y: 595.0,
            },
            Location {
                name: Box::new("36"),
                x: 685.0,
                y: 610.0,
            },
            Location {
                name: Box::new("37"),
                x: 770.0,
                y: 610.0,
            },
            Location {
                name: Box::new("38"),
                x: 795.0,
                y: 645.0,
            },
            Location {
                name: Box::new("39"),
                x: 720.0,
                y: 635.0,
            },
            Location {
                name: Box::new("40"),
                x: 760.0,
                y: 650.0,
            },
            Location {
                name: Box::new("41"),
                x: 475.0,
                y: 960.0,
            },
            Location {
                name: Box::new("42"),
                x: 95.0,
                y: 260.0,
            },
            Location {
                name: Box::new("43"),
                x: 875.0,
                y: 920.0,
            },
            Location {
                name: Box::new("44"),
                x: 700.0,
                y: 500.0,
            },
            Location {
                name: Box::new("45"),
                x: 555.0,
                y: 815.0,
            },
            Location {
                name: Box::new("46"),
                x: 830.0,
                y: 485.0,
            },
            Location {
                name: Box::new("47"),
                x: 1170.0,
                y: 65.0,
            },
            Location {
                name: Box::new("48"),
                x: 830.0,
                y: 610.0,
            },
            Location {
                name: Box::new("49"),
                x: 605.0,
                y: 625.0,
            },
            Location {
                name: Box::new("50"),
                x: 595.0,
                y: 360.0,
            },
            Location {
                name: Box::new("51"),
                x: 1340.0,
                y: 725.0,
            },
            Location {
                name: Box::new("52"),
                x: 1740.0,
                y: 245.0,
            },
        ];
        Self { random, locations }
    }

    fn cost_of_arc(&self, from: usize, to: usize) -> f64 {
        let lhs = &self.locations[from];
        let rhs = &self.locations[to];
        let x = (lhs.x - rhs.x) * (lhs.x - rhs.x);
        let y = (lhs.y - rhs.y) * (lhs.y - rhs.y);
        (x + y).sqrt()
    }
}

impl<'a> Genetic<TspGenome> for TspGenetic<'a> {
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
