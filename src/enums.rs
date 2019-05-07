//=============================================================================================
// Macros
//=============================================================================================

/// Asserts that a [Result](https://doc.rust-lang.org/std/result/enum.Result.html) is `Ok`
/// 
/// ### Parameters
/// 
/// - `&result` A reference to a result.
/// - `&value` ***(optional)*** A reference to an item to compare to `Ok`'s inner value.
/// 
/// ### Dependencies
/// 
/// - `value` must be comparable to `Ok`'s inner value.
/// 
/// ### Examples
///
/// **Check for `Ok` only:**
/// ```rust
/// use totems::assert_ok;
/// let result = "5".parse::<u32>();
/// assert_ok!(&result)
/// ```
/// **Check for `Ok` and correct inner value:**
/// ```rust
/// use totems::assert_ok;
/// let result = "5".parse::<u32>();
/// assert_ok!(&result, value == &5);
/// assert_ok!(&result, value != &0);
/// assert_ok!(&result, value <  &6);
/// assert_ok!(&result, value <= &6);
/// assert_ok!(&result, value >  &4);
/// assert_ok!(&result, value >= &4);
/// ```
/// 
/// ### Example Error Messages
/// 
/// ```text
/// thread 'main' panicked at 'assertion failed: (&result is Ok(_))
///  &result: Err(ParseIntError { kind: InvalidDigit })
/// ', src/enums.rs:498:9
/// ```
/// ```text
/// thread 'main' panicked at 'assertion failed: (Ok(left) => { left <= right })
///   left: 5
///  right: 4
/// ', src/enums.rs:465:9
/// ```
#[macro_export]
macro_rules! assert_ok {
    ($result:expr) => {{
        if let Err(_) = $result {
            panic!(
                "assertion failed: ({0} is Ok(_))\n {0}: {1:?}\n",
                stringify!($result),
                $result,
            );
        }
    }};
    ($result:expr, value == $value:expr) => {{
        assert_ok!($result);
        if let Ok(val) = $result {
            if val != $value {
                panic!(
                    "assertion failed: (Ok(left) => {{ left == right }})\n  left: {:?}\n right: {:?}\n",
                    val,
                    $value,
                )
            }
        }
    }};
    ($result:expr, value != $value:expr) => {{
        assert_ok!($result);
        if let Ok(val) = $result {
            if val == $value {
                panic!(
                    "assertion failed: (Ok(left) => {{ left != right }})\n  left: {:?}\n right: {:?}\n",
                    val,
                    $value,
                )
            }
        }
    }};
    ($result:expr, value < $value:expr) => {{
        assert_ok!($result);
        if let Ok(val) = $result {
            if val >= $value {
                panic!(
                    "assertion failed: (Ok(left) => {{ left < right }})\n  left: {:?}\n right: {:?}\n",
                    val,
                    $value,
                )
            }
        }
    }};
    ($result:expr, value <= $value:expr) => {{
        assert_ok!($result);
        if let Ok(val) = $result {
            if val > $value {
                panic!(
                    "assertion failed: (Ok(left) => {{ left <= right }})\n  left: {:?}\n right: {:?}\n",
                    val,
                    $value,
                )
            }
        }
    }};
    ($result:expr, value > $value:expr) => {{
        assert_ok!($result);
        if let Ok(val) = $result {
            if val <= $value {
                panic!(
                    "assertion failed: (Ok(left) => {{ left > right }})\n  left: {:?}\n right: {:?}\n",
                    val,
                    $value,
                )
            }
        }
    }};
    ($result:expr, value >= $value:expr) => {{
        assert_ok!($result);
        if let Ok(val) = $result {
            if val < $value {
                panic!(
                    "assertion failed: (Ok(left) => {{ left >= right }})\n  left: {:?}\n right: {:?}\n",
                    val,
                    $value,
                )
            }
        }
    }};
}

/// Asserts that a [Result](https://doc.rust-lang.org/std/result/enum.Result.html) is `Err`
/// 
/// ### Parameters
/// 
/// - `&result` A reference to a result.
/// - `&value` ***(optional)*** A reference to an item to compare to `Err`'s inner value.
/// 
/// ### Dependencies
/// 
/// - `value` must be comparable to `Err`'s inner type.
/// 
/// ### Examples
///
/// **Check for `Err` only:**
/// ```rust
/// use totems::assert_err;
/// let result = "z".parse::<u32>();
/// assert_err!(&result);
/// ```
/// **Check for `Err` and correct inner value:**
/// ```rust
/// use totems::assert_err;
/// let result: Result<(), u32> = Err(5);
/// assert_err!(&result, value == &5);
/// assert_err!(&result, value != &0);
/// assert_err!(&result, value <  &6);
/// assert_err!(&result, value <= &5);
/// assert_err!(&result, value >  &4);
/// assert_err!(&result, value >= &5);
/// ```
/// 
/// ### Example Error Messages
/// 
/// ```text
/// thread 'enums::err::is_err' panicked at 'assertion failed: (result is Err(_))
///  result: Ok(5)
/// ', src/enums.rs:574:9
/// ```
/// ```text
/// thread 'enums::err::eq_incorrect' panicked at 'assertion failed: (Err(left) => { left == right })
///   left: "This message matches."
///  right: "This message doesn\'t match."
/// ', src/enums.rs:491:9
/// ```
#[macro_export]
macro_rules! assert_err {
    ($result:expr) => {{
        if let Ok(_) = $result {
            panic!(
                "assertion failed: ({0} is Err(_))\n {0}: {1:?}\n",
                stringify!($result),
                $result,
            );
        }
    }};
    ($result:expr, value == $value:expr) => {{
        assert_err!($result);
        if let Err(val) = $result {
            if val != $value {
                panic!(
                    "assertion failed: (Err(left) => {{ left == right }})\n  left: {:?}\n right: {:?}\n",
                    val,
                    $value,
                )
            }
        }
    }};
    ($result:expr, value != $value:expr) => {{
        assert_err!($result);
        if let Err(val) = $result {
            if val == $value {
                panic!(
                    "assertion failed: (Err(left) => {{ left != right }})\n  left: {:?}\n right: {:?}\n",
                    val,
                    $value,
                )
            }
        }
    }};
    ($result:expr, value < $value:expr) => {{
        assert_err!($result);
        if let Err(val) = $result {
            if val >= $value {
                panic!(
                    "assertion failed: (Err(left) => {{ left < right }})\n  left: {:?}\n right: {:?}\n",
                    val,
                    $value,
                )
            }
        }
    }};
    ($result:expr, value <= $value:expr) => {{
        assert_err!($result);
        if let Err(val) = $result {
            if val > $value {
                panic!(
                    "assertion failed: (Err(left) => {{ left <= right }})\n  left: {:?}\n right: {:?}\n",
                    val,
                    $value,
                )
            }
        }
    }};
    ($result:expr, value > $value:expr) => {{
        assert_err!($result);
        if let Err(val) = $result {
            if val <= $value {
                panic!(
                    "assertion failed: (Err(left) => {{ left > right }})\n  left: {:?}\n right: {:?}\n",
                    val,
                    $value,
                )
            }
        }
    }};
    ($result:expr, value >= $value:expr) => {{
        assert_err!($result);
        if let Err(val) = $result {
            if val < $value {
                panic!(
                    "assertion failed: (Err(left) => {{ left >= right }})\n  left: {:?}\n right: {:?}\n",
                    val,
                    $value,
                )
            }
        }
    }};
}

/// Asserts that an [Option](https://doc.rust-lang.org/std/option/enum.Option.html) is `Some`
/// 
/// ### Parameters
/// 
/// - `&option` A reference to an `Option`.
/// - `&value` ***(optional)*** A reference to an item to compare to `Some`'s inner value.
/// 
/// ### Dependencies
/// 
/// - `value` must be comparable to `Ok`'s inner value.
/// 
/// ### Examples
///
/// **Check for `Some` only:**
/// ```rust
/// use totems::assert_some;
/// let option = "5".parse::<u32>().ok();
/// assert_some!(&option);
/// ```
/// **Check for `Some` and correct inner value:**
/// ```rust
/// use totems::assert_some;
/// let option = "5".parse::<u32>().ok();
/// assert_some!(&option, value == &5);
/// assert_some!(&option, value != &0);
/// assert_some!(&option, value <  &6);
/// assert_some!(&option, value <= &6);
/// assert_some!(&option, value >  &4);
/// assert_some!(&option, value >= &4);
/// ```
/// 
/// ### Example Error Messages
/// 
/// ```text
/// thread 'main' panicked at 'assertion failed: (&option is Some(_))
///  &option: None
/// ', src/enums.rs:699:9
/// ```
/// ```text
/// thread 'main' panicked at 'assertion failed: (Some(left) => { left > right })
///   left: 5
///  right: 5
/// ', src/enums.rs:679:9
/// ```
#[macro_export]
macro_rules! assert_some {
    ($option:expr) => {{
        if let None = $option {
            panic!(
                "assertion failed: ({0} is Some(_))\n {0}: {1:?}\n",
                stringify!($option),
                $option,
            );
        }
    }};
    ($option:expr, value == $value:expr) => {{
        assert_some!($option);
        if let Some(val) = $option {
            if val != $value {
                panic!(
                    "assertion failed: (Some(left) => {{ left == right }})\n  left: {:?}\n right: {:?}\n",
                    val,
                    $value,
                )
            }
        }
    }};
    ($option:expr, value != $value:expr) => {{
        assert_some!($option);
        if let Some(val) = $option {
            if val == $value {
                panic!(
                    "assertion failed: (Some(left) => {{ left != right }})\n  left: {:?}\n right: {:?}\n",
                    val,
                    $value,
                )
            }
        }
    }};
    ($option:expr, value < $value:expr) => {{
        assert_some!($option);
        if let Some(val) = $option {
            if val >= $value {
                panic!(
                    "assertion failed: (Some(left) => {{ left < right }})\n  left: {:?}\n right: {:?}\n",
                    val,
                    $value,
                )
            }
        }
    }};
    ($option:expr, value <= $value:expr) => {{
        assert_some!($option);
        if let Some(val) = $option {
            if val > $value {
                panic!(
                    "assertion failed: (Some(left) => {{ left <= right }})\n  left: {:?}\n right: {:?}\n",
                    val,
                    $value,
                )
            }
        }
    }};
    ($option:expr, value > $value:expr) => {{
        assert_some!($option);
        if let Some(val) = $option {
            if val <= $value {
                panic!(
                    "assertion failed: (Some(left) => {{ left > right }})\n  left: {:?}\n right: {:?}\n",
                    val,
                    $value,
                )
            }
        }
    }};
    ($option:expr, value >= $value:expr) => {{
        assert_some!($option);
        if let Some(val) = $option {
            if val < $value {
                panic!(
                    "assertion failed: (Some(left) => {{ left >= right }})\n  left: {:?}\n right: {:?}\n",
                    val,
                    $value,
                )
            }
        }
    }};
}

/// Asserts that an [Option](https://doc.rust-lang.org/std/option/enum.Option.html) is `None`
/// 
/// ### Parameters
/// 
/// - `&option` A reference to an `Option`.
/// 
/// ### Examples
/// 
/// ```rust
/// use totems::assert_none;
/// let option = "z".parse::<u32>().ok();
/// assert_none!(&option);
/// ```
/// 
/// ### Example Error Messages
/// 
/// ```text
/// thread 'enums::none::is_some' panicked at 'assertion failed: (&option is None)
///  &option: Some(5)
/// ', src/enums.rs:743:9
/// ```
#[macro_export]
macro_rules! assert_none {
    ($option:expr) => {{
        if let Some(_) = $option {
            panic!(
                "assertion failed: ({0} is None)\n {0}: {1:?}\n",
                stringify!($option),
                $option,
            );
        }
    }};
}

//=============================================================================================
// Unit Tests
//=============================================================================================

#[cfg(test)]
mod ok {
    #[test]
    fn stand_alone() {
        let result = "5".parse::<u32>();
        assert_ok!(&result);
    }

    #[test]
    fn eq_correct() {
        let result = "5".parse::<u32>();
        assert_ok!(&result, value == &5);
    }

    #[test]
    #[should_panic]
    fn eq_incorrect() {
        let result = "5".parse::<u32>();
        assert_ok!(&result, value == &2);
    }

    #[test]
    fn ne_correct() {
        let result = "5".parse::<u32>();
        assert_ok!(&result, value != &2);
    }

    #[test]
    #[should_panic]
    fn ne_incorrect() {
        let result = "5".parse::<u32>();
        assert_ok!(&result, value != &5);
    }

    #[test]
    fn lt_correct() {
        let result = "5".parse::<u32>();
        assert_ok!(&result, value < &6);
    }

    #[test]
    #[should_panic]
    fn lt_incorrect() {
        let result = "5".parse::<u32>();
        assert_ok!(&result, value < &5);
    }

    #[test]
    fn le_correct() {
        let result = "5".parse::<u32>();
        assert_ok!(&result, value <= &5);
    }

    #[test]
    #[should_panic]
    fn le_incorrect() {
        let result = "5".parse::<u32>();
        assert_ok!(&result, value <= &4);
    }

    #[test]
    fn gt_correct() {
        let result = "5".parse::<u32>();
        assert_ok!(&result, value > &4);
    }

    #[test]
    #[should_panic]
    fn gt_incorrect() {
        let result = "5".parse::<u32>();
        assert_ok!(&result, value > &5);
    }

    #[test]
    fn ge_correct() {
        let result = "5".parse::<u32>();
        assert_ok!(&result, value >= &5);
    }

    #[test]
    #[should_panic]
    fn ge_incorrect() {
        let result = "5".parse::<u32>();
        assert_ok!(&result, value >= &6);
    }

    #[test]
    #[should_panic]
    fn is_err() {
        let result = "z".parse::<u32>();
        assert_ok!(&result, value == &5);
    }
}

#[cfg(test)]
mod err {
    #[test]
    fn stand_alone() {
        let result = "z".parse::<u32>();
        assert_err!(result);
    }

    #[test]
    fn eq_correct() {
        let result: Result<(), &str> = Err("This message matches.");
        let err = "This message matches.";
        assert_err!(&result, value == &err);
    }

    #[test]
    #[should_panic]
    fn eq_incorrect() {
        let result: Result<(), &str> = Err("This message matches.");
        let err = "This message doesn't match.";
        assert_err!(&result, value == &err);
    }

    #[test]
    fn ne_correct() {
        let result: Result<(), &str> = Err("This message matches.");
        let err = "This message does not match.";
        assert_err!(&result, value != &err);
    }

    #[test]
    #[should_panic]
    fn ne_incorrect() {
        let result: Result<(), &str> = Err("This message matches.");
        let err = "This message matches.";
        assert_err!(&result, value != &err);
    }

    #[test]
    fn lt_correct() {
        let result: Result<(), u32> = Err(5);
        let err = 6;
        assert_err!(&result, value < &err);
    }

    #[test]
    #[should_panic]
    fn lt_incorrect() {
        let result: Result<(), u32> = Err(5);
        let err = 5;
        assert_err!(&result, value < &err);
    }

    #[test]
    fn le_correct() {
        let result: Result<(), u32> = Err(5);
        let err = 5;
        assert_err!(&result, value <= &err);
    }

    #[test]
    #[should_panic]
    fn le_incorrect() {
        let result: Result<(), u32> = Err(5);
        let err = 4;
        assert_err!(&result, value <= &err);
    }


    #[test]
    fn gt_correct() {
        let result: Result<(), u32> = Err(5);
        let err = 4;
        assert_err!(&result, value > &err);
    }

    #[test]
    #[should_panic]
    fn gt_incorrect() {
        let result: Result<(), u32> = Err(5);
        let err = 5;
        assert_err!(&result, value > &err);
    }

    #[test]
    fn ge_correct() {
        let result: Result<(), u32> = Err(5);
        let err = 5;
        assert_err!(&result, value >= &err);
    }

    #[test]
    #[should_panic]
    fn ge_incorrect() {
        let result: Result<(), u32> = Err(5);
        let err = 6;
        assert_err!(&result, value >= &err);
    }

    #[test]
    #[should_panic]
    fn is_err() {
        let result = "5".parse::<u32>();
        assert_err!(result);
    }
}

#[cfg(test)]
mod some {
    #[test]
    fn stand_alone() {
        let option = "5".parse::<u32>().ok();
        assert_some!(&option);
    }

    #[test]
    fn eq_correct() {
        let option = "5".parse::<u32>().ok();
        assert_some!(&option, value == &5);
    }

    #[test]
    #[should_panic]
    fn eq_incorrect() {
        let option = "5".parse::<u32>().ok();
        assert_some!(&option, value == &2);
    }

    #[test]
    fn ne_correct() {
        let option = "5".parse::<u32>().ok();
        assert_some!(&option, value != &2);
    }

    #[test]
    #[should_panic]
    fn ne_incorrect() {
        let option = "5".parse::<u32>().ok();
        assert_some!(&option, value != &5);
    }

    #[test]
    fn lt_correct() {
        let option = "5".parse::<u32>().ok();
        assert_some!(&option, value < &6);
    }

    #[test]
    #[should_panic]
    fn lt_incorrect() {
        let option = "5".parse::<u32>().ok();
        assert_some!(&option, value < &5);
    }

    #[test]
    fn le_correct() {
        let option = "5".parse::<u32>().ok();
        assert_some!(&option, value <= &5);
    }

    #[test]
    #[should_panic]
    fn le_incorrect() {
        let option = "5".parse::<u32>().ok();
        assert_some!(&option, value <= &4);
    }

    #[test]
    fn gt_correct() {
        let option = "5".parse::<u32>().ok();
        assert_some!(&option, value > &4);
    }

    #[test]
    #[should_panic]
    fn gt_incorrect() {
        let option = "5".parse::<u32>().ok();
        assert_some!(&option, value > &5);
    }

    #[test]
    fn ge_correct() {
        let option = "5".parse::<u32>().ok();
        assert_some!(&option, value >= &5);
    }

    #[test]
    #[should_panic]
    fn ge_incorrect() {
        let option = "5".parse::<u32>().ok();
        assert_some!(&option, value >= &6);
    }

    #[test]
    #[should_panic]
    fn is_none() {
        let option = "z".parse::<u32>().ok();
        assert_some!(&option, value == &5);
    }
}

#[cfg(test)]
mod none {
    #[test]
    fn stand_alone() {
        let option = "z".parse::<u32>().ok();
        assert_none!(&option);
    }

    #[test]
    #[should_panic]
    fn is_some() {
        let option = "5".parse::<u32>().ok();
        assert_none!(&option);
    }
}