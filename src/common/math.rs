pub(crate) fn mean(costs: &[f64]) -> f64 {
    costs.iter().fold(0.0, |acc, x| acc + x) / costs.len() as f64
}

pub(crate) fn largest(costs: &[f64]) -> f64 {
    costs
        .iter()
        .fold(0.0, |acc, x| if *x > acc { *x } else { acc })
}
