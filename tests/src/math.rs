// BUG: do not use for production code.
pub fn round(value: f64, base: usize, places: usize) -> f64 {
    if places == 0 {
        value.trunc()
    } else {
        let scale = (base * places) as f64;
        (value * scale).floor() / scale as f64
    }
}
