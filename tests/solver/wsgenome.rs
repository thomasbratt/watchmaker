use crate::common::make_random_string;
use rand::distributions::Alphanumeric;
use rand::{Rng, RngCore};
use watchmaker::Genetic;

#[derive(Debug)]
pub(crate) struct WSGenome(String);

static TARGET: &str = "weasel";
static LENGTH: usize = TARGET.len();

impl Genetic<WSGenome> for WSGenome {
    fn initialize(random: &mut Box<dyn RngCore>) -> WSGenome {
        // make_random_string(random, LENGTH)
        // let s: String = random
        //     .sample_iter(&Alphanumeric)
        //     .take(LENGTH)
        //     .map(char::from)
        //     .collect::<String>()
        //     .to_string();
        WSGenome(make_random_string(random, LENGTH))
    }

    fn evaluate(genome: WSGenome) -> f64 {
        todo!()
    }

    fn crossover(random: &mut Box<dyn RngCore>, lhs: WSGenome, rhs: WSGenome) -> WSGenome {
        todo!()
    }

    fn mutate(random: &mut Box<dyn RngCore>, original: WSGenome) -> WSGenome {
        todo!()
    }
}
