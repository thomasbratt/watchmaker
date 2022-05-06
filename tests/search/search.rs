use rand::Rng;
use watchmaker::*;

#[test]
fn search_finds_result_for_simple_test_case() {
    let result = search(
        Box::new(PeakGenetic::new(make_random_from_seed(123))),
        Box::new(TournamentSelector::default()),
        None,
        make_random_from_seed(456),
        &SearchSettings::default(),
    );

    eprintln!("{:?}", result);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.as_ref().ok().unwrap().best_cost(), 0.0);
    assert_eq!(result.as_ref().ok().unwrap().best_genome().0, TARGET);
}

#[derive(Clone, Debug, PartialEq)]
#[doc(hidden)]
pub struct PeakGenome(pub f64);

#[doc(hidden)]
pub static TARGET: f64 = 100.0;

#[doc(hidden)]
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
        PeakGenome(self.random.gen_range(0.0..200.0))
    }

    fn evaluate(&mut self, genome: &PeakGenome) -> f64 {
        round((TARGET - genome.0).abs(), 10, 2)
    }

    fn crossover(&mut self, lhs: &PeakGenome, rhs: &PeakGenome) -> PeakGenome {
        PeakGenome(round((lhs.0 + rhs.0) / 2.0, 10, 2))
    }

    fn mutate(&mut self, original: &PeakGenome) -> PeakGenome {
        PeakGenome(original.0 + self.random.gen_range(-10.0..10.0))
    }
}
