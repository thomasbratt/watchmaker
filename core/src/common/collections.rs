#[doc(hidden)]
pub fn make_vec<T, F: FnMut() -> T>(n: usize, repeater: F) -> Vec<T> {
    std::iter::repeat_with(repeater).take(n).collect()
}
