pub(crate) fn mean(values: &[f64]) -> f64 {
    values.iter().fold(0.0, |acc, x| acc + x) / values.len() as f64
}

pub(crate) fn largest(values: &[f64]) -> f64 {
    values
        .iter()
        .fold(0.0, |acc, x| if *x > acc { *x } else { acc })
}
