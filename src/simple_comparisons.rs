#[macro_export]
macro_rules! assert_lt {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val < *right_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left < right)`
  left: `{:?}`,
 right: `{:?}`"#, &*left_val, &*right_val)
                }
            }
        }
    });
    ($left:expr, $right:expr,) => ({
        assert_lt!($left, $right)
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val < *right_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left < right)`
  left: `{:?}`,
 right: `{:?}`: {}"#, &*left_val, &*right_val,
                           format_args!($($arg)+))
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn less_than_correct() {
        assert_lt!(5, 6);
    }
    #[test]
    fn less_than_trailing_comma() {
        assert_lt!(5, 6,);
    }
    #[test]
    fn less_than_format_string() {
        let x = 5;
        let y = 6;
        assert_lt!(x, y, "{} is less than {}", x, y);
    }
    #[test]
    #[should_panic]
    fn less_than_incorrect() {
        assert_lt!(6, 5);
    }
}
