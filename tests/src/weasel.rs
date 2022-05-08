use crate::random::*;
use rand::{thread_rng, Rng};
use watchmaker::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WSGenome(pub String);

pub static TARGET: &str = "METHINKS IT IS LIKE A WEASEL";

#[derive(Default)]
pub struct WSGenetic {}

impl Genetic<WSGenome> for WSGenetic {
    fn initialize(&self) -> WSGenome {
        WSGenome(make_random_string(TARGET.len()))
    }

    fn evaluate(&self, genome: &WSGenome) -> f64 {
        let lhs_str = TARGET;
        let rhs_str = genome.0.as_str();
        lhs_str.chars().zip(rhs_str.chars()).fold(0.0, |acc, x| {
            acc + (x.0 as i16 as f64 - x.1 as i16 as f64).abs()
        })
    }

    fn crossover(&self, lhs: &WSGenome, rhs: &WSGenome) -> WSGenome {
        let lhs_str = lhs.0.as_str();
        let rhs_str = rhs.0.as_str();
        let crossover_index = rand::thread_rng().gen_range(0..lhs_str.len() - 1);
        let result = lhs_str
            .chars()
            .zip(rhs_str.chars())
            .enumerate()
            .map(|(i, (a, b))| if i > crossover_index { a } else { b });
        WSGenome(String::from_iter(result))
    }

    fn mutate(&self, original: &WSGenome) -> WSGenome {
        let chars = original.0.as_str();
        let mutate_index = thread_rng().gen_range(0..chars.len() as usize);
        let chars_result = chars.chars().into_iter().enumerate().map(|(i, c)| {
            if i == mutate_index {
                char::from(thread_rng().sample(&RandomStringDistribution::new(CHARACTERS)))
            } else {
                c
            }
        });
        WSGenome(String::from_iter(chars_result))
    }
}
