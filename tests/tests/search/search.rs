use rand::Rng;
use tests::assert_between;
use tests::round;
use watchmaker::*;

#[test]
fn search_finds_result_for_simple_test_case() {
    let result = search(
        Box::new(PeakGenetic::new()),
        Box::new(TourniquetSelector::default()),
        None,
        &SearchSettings::default(),
    );

    assert_eq!(result.is_ok(), true);

    let success = result.as_ref().ok().unwrap();
    assert_between!(success.best_cost(), 0.0, 0.1);
    assert_between!(success.best_genome().0, TARGET - 1.0, TARGET + 1.0);
}

#[derive(Clone, Debug, PartialEq)]
#[doc(hidden)]
pub struct PeakGenome(pub f64);

#[doc(hidden)]
pub static TARGET: f64 = 100.0;

#[doc(hidden)]
pub struct PeakGenetic {}

impl PeakGenetic {
    pub fn new() -> Self {
        Self {}
    }
}

impl Genetic<PeakGenome> for PeakGenetic {
    fn initialize(&self) -> PeakGenome {
        PeakGenome(rand::thread_rng().gen_range(0.0..200.0))
    }

    fn evaluate(&self, genome: &PeakGenome) -> f64 {
        round((TARGET - genome.0).abs(), 10, 2)
    }

    fn crossover(&self, lhs: &PeakGenome, rhs: &PeakGenome) -> PeakGenome {
        PeakGenome(round((lhs.0 + rhs.0) / 2.0, 10, 2))
    }

    fn mutate(&self, original: &PeakGenome) -> PeakGenome {
        PeakGenome(original.0 + rand::thread_rng().gen_range(-10.0..10.0))
    }
}
