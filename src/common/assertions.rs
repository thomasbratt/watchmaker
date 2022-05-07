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
    }}; // TODO:
        // ($left:expr, $right:expr, $($arg:tt)+) => ({
        //     match (&$left, &$right) {
        //         (left_val, right_val) => {
        //             if !(*left_val == *right_val) {
        //                 let kind = $crate::panicking::AssertKind::Eq;
        //                 // The reborrows below are intentional. Without them, the stack slot for the
        //                 // borrow is initialized even before the values are compared, leading to a
        //                 // noticeable slow down.
        //                 $crate::panicking::assert_failed(kind, &*left_val, &*right_val, $crate::option::Option::Some($crate::format_args!($($arg)+)));
        //             }
        //         }
        //     }
        // });
}
