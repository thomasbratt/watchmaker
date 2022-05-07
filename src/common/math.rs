pub fn round(value: f64, base: usize, places: usize) -> f64 {
    if places == 0 {
        value.trunc()
    } else {
        let scale = (base * places) as f64;
        (value * scale).floor() / scale as f64
    }
}

pub(crate) fn mean(values: &[f64]) -> f64 {
    values.iter().fold(0.0, |acc, x| acc + x) / values.len() as f64
}

pub(crate) fn largest(values: &[f64]) -> f64 {
    values
        .iter()
        .fold(0.0, |acc, x| if *x > acc { *x } else { acc })
}
