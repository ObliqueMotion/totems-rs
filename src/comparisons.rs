//=============================================================================================
// Macros
//=============================================================================================

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

#[macro_export]
macro_rules! assert_le {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val <= *right_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left <= right)`
  left: `{:?}`,
 right: `{:?}`"#, &*left_val, &*right_val)
                }
            }
        }
    });
    ($left:expr, $right:expr,) => ({
        assert_le!($left, $right)
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val <= *right_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left <= right)`
  left: `{:?}`,
 right: `{:?}`: {}"#, &*left_val, &*right_val,
                           format_args!($($arg)+))
                }
            }
        }
    });
}

#[macro_export]
macro_rules! assert_gt {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val > *right_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left > right)`
  left: `{:?}`,
 right: `{:?}`"#, &*left_val, &*right_val)
                }
            }
        }
    });
    ($left:expr, $right:expr,) => ({
        assert_gt!($left, $right)
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val > *right_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left > right)`
  left: `{:?}`,
 right: `{:?}`: {}"#, &*left_val, &*right_val,
                           format_args!($($arg)+))
                }
            }
        }
    });
}

#[macro_export]
macro_rules! assert_ge {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val >= *right_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left >= right)`
  left: `{:?}`,
 right: `{:?}`"#, &*left_val, &*right_val)
                }
            }
        }
    });
    ($left:expr, $right:expr,) => ({
        assert_ge!($left, $right)
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val >= *right_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left >= right)`
  left: `{:?}`,
 right: `{:?}`: {}"#, &*left_val, &*right_val,
                           format_args!($($arg)+))
                }
            }
        }
    });
}

//=============================================================================================
// Unit Tests
//=============================================================================================

#[cfg(test)]
mod lt {
    #[test]
    fn correct() {
        assert_lt!(5, 7);
        assert_lt!(5, 6);
    }
    #[test]
    fn trailing_comma() {
        assert_lt!(5, 6,);
    }
    #[test]
    fn format_string() {
        let x = 5;
        let y = 6;
        assert_lt!(x, y, "{} is less than {}", x, y);
    }
    #[test]
    #[should_panic]
    fn incorrect() {
        assert_lt!(5, 5);
    }
}

#[cfg(test)]
mod le {
    #[test]
    fn correct() {
        assert_le!(5, 6);
        assert_le!(5, 5);
    }
    #[test]
    fn trailing_comma() {
        assert_le!(5, 6,);
        assert_le!(5, 5,);
    }
    #[test]
    fn format_string() {
        let x = 5;
        let y = 6;
        assert_le!(x, y, "{} is less or equal to {}", x, y);
    }
    #[test]
    #[should_panic]
    fn incorrect() {
        assert_le!(6, 5);
    }
}

#[cfg(test)]
mod gt {
    #[test]
    fn correct() {
        assert_gt!(7, 5);
        assert_gt!(6, 5);
    }
    #[test]
    fn trailing_comma() {
        assert_gt!(7, 5,);
        assert_gt!(6, 5,);
    }
    #[test]
    fn format_string() {
        let x = 7;
        let y = 5;
        assert_gt!(x, y, "{} is greater than {}", x, y);
    }
    #[test]
    #[should_panic]
    fn incorrect() {
        assert_gt!(5, 5);
    }
}

#[cfg(test)]
mod ge {
    #[test]
    fn correct() {
        assert_ge!(6, 6);
        assert_ge!(5, 5);
    }
    #[test]
    fn trailing_comma() {
        assert_ge!(6, 6,);
        assert_ge!(5, 5,);
    }
    #[test]
    fn format_string() {
        let x = 6;
        let y = 5;
        assert_ge!(x, y, "{} is greater or equal to {}", x, y);
    }
    #[test]
    #[should_panic]
    fn incorrect() {
        assert_ge!(5, 6);
    }
}
