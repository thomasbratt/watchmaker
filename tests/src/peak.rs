use rand::Rng;
use watchmaker::*;

#[derive(Clone, Debug, PartialEq)]
pub struct PeakGenome(pub f64);

pub const TARGET: f64 = 100.0;

#[derive(Default)]
pub struct PeakGenetic {}

impl Genetic<PeakGenome> for PeakGenetic {
    fn initialize(&self) -> PeakGenome {
        PeakGenome(rand::thread_rng().gen_range(0.0..1_000.0))
    }

    fn evaluate(&self, genome: &PeakGenome) -> f64 {
        (TARGET - genome.0).abs()
    }

    fn crossover(&self, lhs: &PeakGenome, rhs: &PeakGenome) -> PeakGenome {
        PeakGenome((lhs.0 + rhs.0) / 2.0)
    }

    fn mutate(&self, original: &PeakGenome) -> PeakGenome {
        PeakGenome(original.0 + rand::thread_rng().gen_range(-10.0..10.0))
    }
}
