use crate::common::make_random_string;
use watchmaker::{Genetic, Random};

#[derive(Debug)]
pub(crate) struct WSGenome(String);

static TARGET: &str = "weasel";
static LENGTH: usize = TARGET.len();

impl Genetic<WSGenome> for WSGenome {
    fn initialize(random: &mut Random) -> WSGenome {
        WSGenome(make_random_string(random, LENGTH))
    }

    fn evaluate(genome: WSGenome) -> f64 {
        todo!()
    }

    fn crossover(random: &mut Random, lhs: WSGenome, rhs: WSGenome) -> WSGenome {
        todo!()
    }

    fn mutate(random: &mut Random, original: WSGenome) -> WSGenome {
        todo!()
    }
}
