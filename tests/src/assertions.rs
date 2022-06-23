#[macro_export]
macro_rules! assert_between {
    ($left:expr, $rhs_lower:expr, $rhs_upper:expr $(,)?) => {{
        match (&$left, &$rhs_lower, &$rhs_upper) {
            (left_val, right_lower_val, right_upper_val) => {
                if !(*right_lower_val <= *right_upper_val) {
                    panic!(
                        "error in assert_between: lower bound {} should be less than or equal to upper bound {}",
                        &*right_lower_val, &*right_upper_val,
                    );
                }
                if !((*left_val >= *right_lower_val) && (*left_val <= *right_upper_val)) {
                    panic!(
                        "lhs value {} should be between {} and {} (inclusive)",
                        &*left_val, &*right_lower_val, &*right_upper_val,
                    );
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! assert_not_between {
    ($left:expr, $rhs_lower:expr, $rhs_upper:expr $(,)?) => {{
        match (&$left, &$rhs_lower, &$rhs_upper) {
            (left_val, right_lower_val, right_upper_val) => {
                if !(*right_lower_val <= *right_upper_val) {
                    panic!(
                        "error in assert_not_between: lower bound {} should be less than or equal to upper bound {}",
                        &*right_lower_val, &*right_upper_val,
                    );
                }
                if ((*left_val >= *right_lower_val) && (*left_val <= *right_upper_val)) {
                    panic!(
                        "lhs value {} should not be between {} and {} (inclusive)",
                        &*left_val, &*right_lower_val, &*right_upper_val,
                    );
                }
            }
        }
    }};
}
