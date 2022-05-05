use rand::Rng;
use watchmaker::{make_random, search, Genetic, Random, SettingsBuilder};

// Show how the results and progress reporting work.
// The genetic algorithm will search for the number 100.
/// The name comes from the fact that the cost function has its 'peak' (actually its lowest value)
/// at 100.
fn main() {
    let result = search(
        Box::new(PeakGenetic::new(make_random())),
        Some(Box::new(|snapshot| {
            println!("progress snapshot:{:?}", snapshot);
        })),
        make_random(),
        &SettingsBuilder::default()
            .cross_over_candidates(2)
            .mutation_probability(0.1)
            .population_size(30)
            .build()
            .unwrap(),
    );
    println!("{:?}", result);
}

#[derive(Clone, Debug, PartialEq)]
pub struct PeakGenome(pub f64);

pub static TARGET: f64 = 100.0;

pub struct PeakGenetic {
    random: Random,
}

impl PeakGenetic {
    pub fn new(random: Random) -> Self {
        Self { random }
    }
}

impl Genetic<PeakGenome> for PeakGenetic {
    fn initialize(&mut self) -> PeakGenome {
        PeakGenome(self.random.gen_range(0.0..1_000.0))
    }

    fn evaluate(&mut self, genome: &PeakGenome) -> f64 {
        (TARGET - genome.0).abs()
    }

    fn crossover(&mut self, lhs: &PeakGenome, rhs: &PeakGenome) -> PeakGenome {
        PeakGenome((lhs.0 + rhs.0) / 2.0)
    }

    fn mutate(&mut self, original: &PeakGenome) -> PeakGenome {
        PeakGenome(original.0 + self.random.gen_range(-10.0..10.0))
    }
}
