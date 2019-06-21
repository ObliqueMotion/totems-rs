//=============================================================================================
// Macros
//=============================================================================================

/// Asserts that the 0th `item` in a `tuple` has a relationship to some value.
/// 
/// ### Parameters
/// 
/// - `&tuple` A reference to a tuple.
/// - `&val` A reference to a value to compare to the 0th item.
/// 
/// ### Dependencies
/// 
/// - All content must implement [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
/// - `val` must implement PartialEq for the 0th type in the tuple to use `==` or `!=`.
/// - `val` must implement PartialOrd for the 0th type in the tuple to use `<`, `<=`, `>`, `>=`.
/// 
/// ### Example
///
/// ```
/// use totems::assert_0th;
/// let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
/// let x = 1;
/// assert_0th!(&tuple, value == &x); // tuple.0 == x
/// assert_0th!(&tuple, value <= &x);
/// assert_0th!(&tuple, value >= &x);
/// assert_0th!(&tuple, value < &(x + 1));
/// assert_0th!(&tuple, value > &(x - 1));
/// ```
///
/// ### Example Error Messages 
///
/// ```text 
/// thread 'tuples::_00th::le_correct' panicked at 'assertion failed: (tuple.0 <= val)
///     val: 0
/// tuple.0: 1
/// ', src/tuples.rs:2162:9
/// ```
#[macro_export]
macro_rules! assert_0th {
    ($tuple:expr, value == $val:expr) => {
        if &$tuple.0 != $val {
            panic!("assertion failed: (tuple.0 == val)\n    val: {:?}\ntuple.0: {:?}\n",
                $val,
                $tuple.0,
            );
        }
    };
    ($tuple:expr, value != $val:expr) => {
        if &$tuple.0 == $val {
            panic!("assertion failed: (tuple.0 != val)\n    val: {:?}\ntuple.0: {:?}\n",
                $val,
                $tuple.0,
            );
        }
    };
    ($tuple:expr, value < $val:expr) => {
        if &$tuple.0 >= $val {
            panic!("assertion failed: (tuple.0 < val)\n    val: {:?}\ntuple.0: {:?}\n",
                $val,
                $tuple.0,
            );
        }
    };
    ($tuple:expr, value <= $val:expr) => {
        if &$tuple.0 > $val {
            panic!("assertion failed: (tuple.0 <= val)\n    val: {:?}\ntuple.0: {:?}\n",
                $val,
                $tuple.0,
            );
        }
    };
    ($tuple:expr, value > $val:expr) => {
        if &$tuple.0 <= $val {
            panic!("assertion failed: (tuple.0 > val)\n    val: {:?}\ntuple.0: {:?}\n",
                $val,
                $tuple.0,
            );
        }
    };
    ($tuple:expr, value >= $val:expr) => {
        if &$tuple.0 < $val {
            panic!("assertion failed: (tuple.0 >= val)\n    val: {:?}\ntuple.0: {:?}\n",
                $val,
                $tuple.0,
            );
        }
    };
}

/// Asserts that the 1st `item` in a `tuple` has a relationship to some value.
/// 
/// ### Parameters
/// 
/// - `&tuple` A reference to a tuple.
/// - `&val` A reference to a value to compare to the 1st item.
/// 
/// ### Dependencies
/// 
/// - All content must implement [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
/// - `val` must implement PartialEq for the 1st type in the tuple to use `==` or `!=`.
/// - `val` must implement PartialOrd for the 1st type in the tuple to use `<`, `<=`, `>`, `>=`.
/// 
/// ### Example
///
/// ```
/// use totems::assert_1st;
/// let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
/// let x = 2;
/// assert_1st!(&tuple, value == &x); // tuple.1 == x
/// assert_1st!(&tuple, value <= &x);
/// assert_1st!(&tuple, value >= &x);
/// assert_1st!(&tuple, value < &(x + 1));
/// assert_1st!(&tuple, value > &(x - 1));
/// ```
///
/// ### Example Error Messages 
///
/// ```text 
/// thread 'tuples::_01st::le_correct' panicked at 'assertion failed: (tuple.1 <= val)
///     val: 0
/// tuple.1: 1
/// ', src/tuples.rs:2162:9
/// ```
#[macro_export]
macro_rules! assert_1st {
    ($tuple:expr, value == $val:expr) => {
        if &$tuple.1 != $val {
            panic!("assertion failed: (tuple.1 == val)\n    val: {:?}\ntuple.1: {:?}\n",
                $val,
                $tuple.1,
            );
        }
    };
    ($tuple:expr, value != $val:expr) => {
        if &$tuple.1 == $val {
            panic!("assertion failed: (tuple.1 != val)\n    val: {:?}\ntuple.1: {:?}\n",
                $val,
                $tuple.1,
            );
        }
    };
    ($tuple:expr, value < $val:expr) => {
        if &$tuple.1 >= $val {
            panic!("assertion failed: (tuple.1 < val)\n    val: {:?}\ntuple.1: {:?}\n",
                $val,
                $tuple.1,
            );
        }
    };
    ($tuple:expr, value <= $val:expr) => {
        if &$tuple.1 > $val {
            panic!("assertion failed: (tuple.1 <= val)\n    val: {:?}\ntuple.1: {:?}\n",
                $val,
                $tuple.1,
            );
        }
    };
    ($tuple:expr, value > $val:expr) => {
        if &$tuple.1 <= $val {
            panic!("assertion failed: (tuple.1 > val)\n    val: {:?}\ntuple.1: {:?}\n",
                $val,
                $tuple.1,
            );
        }
    };
    ($tuple:expr, value >= $val:expr) => {
        if &$tuple.1 < $val {
            panic!("assertion failed: (tuple.1 >= val)\n    val: {:?}\ntuple.1: {:?}\n",
                $val,
                $tuple.1,
            );
        }
    };
}

/// Asserts that the 2nd `item` in a `tuple` has a relationship to some value.
/// 
/// ### Parameters
/// 
/// - `&tuple` A reference to a tuple.
/// - `&val` A reference to a value to compare to the 2nd item.
/// 
/// ### Dependencies
/// 
/// - All content must implement [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
/// - `val` must implement PartialEq for the 2nd type in the tuple to use `==` or `!=`.
/// - `val` must implement PartialOrd for the 2nd type in the tuple to use `<`, `<=`, `>`, `>=`.
/// 
/// ### Example
///
/// ```
/// use totems::assert_2nd;
/// let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
/// let x = 3;
/// assert_2nd!(&tuple, value == &x); // tuple.2 == x
/// assert_2nd!(&tuple, value <= &x);
/// assert_2nd!(&tuple, value >= &x);
/// assert_2nd!(&tuple, value < &(x + 1));
/// assert_2nd!(&tuple, value > &(x - 1));
/// ```
///
/// ### Example Error Messages 
///
/// ```text 
/// thread 'tuples::_02nd::le_correct' panicked at 'assertion failed: (tuple.2 <= val)
///     val: 0
/// tuple.2: 1
/// ', src/tuples.rs:2162:9
/// ```
#[macro_export]
macro_rules! assert_2nd {
    ($tuple:expr, value == $val:expr) => {
        if &$tuple.2 != $val {
            panic!("assertion failed: (tuple.2 == val)\n    val: {:?}\ntuple.2: {:?}\n",
                $val,
                $tuple.2,
            );
        }
    };
    ($tuple:expr, value != $val:expr) => {
        if &$tuple.2 == $val {
            panic!("assertion failed: (tuple.2 != val)\n    val: {:?}\ntuple.2: {:?}\n",
                $val,
                $tuple.2,
            );
        }
    };
    ($tuple:expr, value < $val:expr) => {
        if &$tuple.2 >= $val {
            panic!("assertion failed: (tuple.2 < val)\n    val: {:?}\ntuple.2: {:?}\n",
                $val,
                $tuple.2,
            );
        }
    };
    ($tuple:expr, value <= $val:expr) => {
        if &$tuple.2 > $val {
            panic!("assertion failed: (tuple.2 <= val)\n    val: {:?}\ntuple.2: {:?}\n",
                $val,
                $tuple.2,
            );
        }
    };
    ($tuple:expr, value > $val:expr) => {
        if &$tuple.2 <= $val {
            panic!("assertion failed: (tuple.2 > val)\n    val: {:?}\ntuple.2: {:?}\n",
                $val,
                $tuple.2,
            );
        }
    };
    ($tuple:expr, value >= $val:expr) => {
        if &$tuple.2 < $val {
            panic!("assertion failed: (tuple.2 >= val)\n    val: {:?}\ntuple.2: {:?}\n",
                $val,
                $tuple.2,
            );
        }
    };
}

/// Asserts that the 3rd `item` in a `tuple` has a relationship to some value.
/// 
/// ### Parameters
/// 
/// - `&tuple` A reference to a tuple.
/// - `&val` A reference to a value to compare to the 3rd item.
/// 
/// ### Dependencies
/// 
/// - All content must implement [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
/// - `val` must implement PartialEq for the 3rd type in the tuple to use `==` or `!=`.
/// - `val` must implement PartialOrd for the 3rd type in the tuple to use `<`, `<=`, `>`, `>=`.
/// 
/// ### Example
///
/// ```
/// use totems::assert_3rd;
/// let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
/// let x = 4;
/// assert_3rd!(&tuple, value == &x); // tuple.3 == x
/// assert_3rd!(&tuple, value <= &x);
/// assert_3rd!(&tuple, value >= &x);
/// assert_3rd!(&tuple, value < &(x + 1));
/// assert_3rd!(&tuple, value > &(x - 1));
/// ```
///
/// ### Example Error Messages 
///
/// ```text 
/// thread 'tuples::_03rd::le_correct' panicked at 'assertion failed: (tuple.3 <= val)
///     val: 0
/// tuple.3: 1
/// ', src/tuples.rs:2162:9
/// ```
#[macro_export]
macro_rules! assert_3rd {
    ($tuple:expr, value == $val:expr) => {
        if &$tuple.3 != $val {
            panic!("assertion failed: (tuple.3 == val)\n    val: {:?}\ntuple.3: {:?}\n",
                $val,
                $tuple.3,
            );
        }
    };
    ($tuple:expr, value != $val:expr) => {
        if &$tuple.3 == $val {
            panic!("assertion failed: (tuple.3 != val)\n    val: {:?}\ntuple.3: {:?}\n",
                $val,
                $tuple.3,
            );
        }
    };
    ($tuple:expr, value < $val:expr) => {
        if &$tuple.3 >= $val {
            panic!("assertion failed: (tuple.3 < val)\n    val: {:?}\ntuple.3: {:?}\n",
                $val,
                $tuple.3,
            );
        }
    };
    ($tuple:expr, value <= $val:expr) => {
        if &$tuple.3 > $val {
            panic!("assertion failed: (tuple.3 <= val)\n    val: {:?}\ntuple.3: {:?}\n",
                $val,
                $tuple.3,
            );
        }
    };
    ($tuple:expr, value > $val:expr) => {
        if &$tuple.3 <= $val {
            panic!("assertion failed: (tuple.3 > val)\n    val: {:?}\ntuple.3: {:?}\n",
                $val,
                $tuple.3,
            );
        }
    };
    ($tuple:expr, value >= $val:expr) => {
        if &$tuple.3 < $val {
            panic!("assertion failed: (tuple.3 >= val)\n    val: {:?}\ntuple.3: {:?}\n",
                $val,
                $tuple.3,
            );
        }
    };
}

/// Asserts that the 4th `item` in a `tuple` has a relationship to some value.
/// 
/// ### Parameters
/// 
/// - `&tuple` A reference to a tuple.
/// - `&val` A reference to a value to compare to the 4th item.
/// 
/// ### Dependencies
/// 
/// - All content must implement [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
/// - `val` must implement PartialEq for the 4th type in the tuple to use `==` or `!=`.
/// - `val` must implement PartialOrd for the 4th type in the tuple to use `<`, `<=`, `>`, `>=`.
/// 
/// ### Example
///
/// ```
/// use totems::assert_4th;
/// let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
/// let x = 5;
/// assert_4th!(&tuple, value == &x); // tuple.4 == x
/// assert_4th!(&tuple, value <= &x);
/// assert_4th!(&tuple, value >= &x);
/// assert_4th!(&tuple, value < &(x + 1));
/// assert_4th!(&tuple, value > &(x - 1));
/// ```
///
/// ### Example Error Messages 
///
/// ```text 
/// thread 'tuples::_04th::le_correct' panicked at 'assertion failed: (tuple.4 <= val)
///     val: 0
/// tuple.4: 1
/// ', src/tuples.rs:2162:9
/// ```
#[macro_export]
macro_rules! assert_4th {
    ($tuple:expr, value == $val:expr) => {
        if &$tuple.4 != $val {
            panic!("assertion failed: (tuple.4 == val)\n    val: {:?}\ntuple.4: {:?}\n",
                $val,
                $tuple.4,
            );
        }
    };
    ($tuple:expr, value != $val:expr) => {
        if &$tuple.4 == $val {
            panic!("assertion failed: (tuple.4 != val)\n    val: {:?}\ntuple.4: {:?}\n",
                $val,
                $tuple.4,
            );
        }
    };
    ($tuple:expr, value < $val:expr) => {
        if &$tuple.4 >= $val {
            panic!("assertion failed: (tuple.4 < val)\n    val: {:?}\ntuple.4: {:?}\n",
                $val,
                $tuple.4,
            );
        }
    };
    ($tuple:expr, value <= $val:expr) => {
        if &$tuple.4 > $val {
            panic!("assertion failed: (tuple.4 <= val)\n    val: {:?}\ntuple.4: {:?}\n",
                $val,
                $tuple.4,
            );
        }
    };
    ($tuple:expr, value > $val:expr) => {
        if &$tuple.4 <= $val {
            panic!("assertion failed: (tuple.4 > val)\n    val: {:?}\ntuple.4: {:?}\n",
                $val,
                $tuple.4,
            );
        }
    };
    ($tuple:expr, value >= $val:expr) => {
        if &$tuple.4 < $val {
            panic!("assertion failed: (tuple.4 >= val)\n    val: {:?}\ntuple.4: {:?}\n",
                $val,
                $tuple.4,
            );
        }
    };
}

/// Asserts that the 5th `item` in a `tuple` has a relationship to some value.
/// 
/// ### Parameters
/// 
/// - `&tuple` A reference to a tuple.
/// - `&val` A reference to a value to compare to the 5th item.
/// 
/// ### Dependencies
/// 
/// - All content must implement [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
/// - `val` must implement PartialEq for the 5th type in the tuple to use `==` or `!=`.
/// - `val` must implement PartialOrd for the 5th type in the tuple to use `<`, `<=`, `>`, `>=`.
/// 
/// ### Example
///
/// ```
/// use totems::assert_5th;
/// let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
/// let x = 6;
/// assert_5th!(&tuple, value == &x); // tuple.5 == x
/// assert_5th!(&tuple, value <= &x);
/// assert_5th!(&tuple, value >= &x);
/// assert_5th!(&tuple, value < &(x + 1));
/// assert_5th!(&tuple, value > &(x - 1));
/// ```
///
/// ### Example Error Messages 
///
/// ```text 
/// thread 'tuples::_05th::le_correct' panicked at 'assertion failed: (tuple.5 <= val)
///     val: 0
/// tuple.5: 1
/// ', src/tuples.rs:2162:9
/// ```
#[macro_export]
macro_rules! assert_5th {
    ($tuple:expr, value == $val:expr) => {
        if &$tuple.5 != $val {
            panic!("assertion failed: (tuple.5 == val)\n    val: {:?}\ntuple.5: {:?}\n",
                $val,
                $tuple.5,
            );
        }
    };
    ($tuple:expr, value != $val:expr) => {
        if &$tuple.5 == $val {
            panic!("assertion failed: (tuple.5 != val)\n    val: {:?}\ntuple.5: {:?}\n",
                $val,
                $tuple.5,
            );
        }
    };
    ($tuple:expr, value < $val:expr) => {
        if &$tuple.5 >= $val {
            panic!("assertion failed: (tuple.5 < val)\n    val: {:?}\ntuple.5: {:?}\n",
                $val,
                $tuple.5,
            );
        }
    };
    ($tuple:expr, value <= $val:expr) => {
        if &$tuple.5 > $val {
            panic!("assertion failed: (tuple.5 <= val)\n    val: {:?}\ntuple.5: {:?}\n",
                $val,
                $tuple.5,
            );
        }
    };
    ($tuple:expr, value > $val:expr) => {
        if &$tuple.5 <= $val {
            panic!("assertion failed: (tuple.5 > val)\n    val: {:?}\ntuple.5: {:?}\n",
                $val,
                $tuple.5,
            );
        }
    };
    ($tuple:expr, value >= $val:expr) => {
        if &$tuple.5 < $val {
            panic!("assertion failed: (tuple.5 >= val)\n    val: {:?}\ntuple.5: {:?}\n",
                $val,
                $tuple.5,
            );
        }
    };
}

/// Asserts that the 6th `item` in a `tuple` has a relationship to some value.
/// 
/// ### Parameters
/// 
/// - `&tuple` A reference to a tuple.
/// - `&val` A reference to a value to compare to the 6th item.
/// 
/// ### Dependencies
/// 
/// - All content must implement [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
/// - `val` must implement PartialEq for the 6th type in the tuple to use `==` or `!=`.
/// - `val` must implement PartialOrd for the 6th type in the tuple to use `<`, `<=`, `>`, `>=`.
/// 
/// ### Example
///
/// ```
/// use totems::assert_6th;
/// let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
/// let x = 7;
/// assert_6th!(&tuple, value == &x); // tuple.6 == x
/// assert_6th!(&tuple, value <= &x);
/// assert_6th!(&tuple, value >= &x);
/// assert_6th!(&tuple, value < &(x + 1));
/// assert_6th!(&tuple, value > &(x - 1));
/// ```
///
/// ### Example Error Messages 
///
/// ```text 
/// thread 'tuples::_06th::le_correct' panicked at 'assertion failed: (tuple.6 <= val)
///     val: 0
/// tuple.6: 1
/// ', src/tuples.rs:2162:9
/// ```
#[macro_export]
macro_rules! assert_6th {
    ($tuple:expr, value == $val:expr) => {
        if &$tuple.6 != $val {
            panic!("assertion failed: (tuple.6 == val)\n    val: {:?}\ntuple.6: {:?}\n",
                $val,
                $tuple.6,
            );
        }
    };
    ($tuple:expr, value != $val:expr) => {
        if &$tuple.6 == $val {
            panic!("assertion failed: (tuple.6 != val)\n    val: {:?}\ntuple.6: {:?}\n",
                $val,
                $tuple.6,
            );
        }
    };
    ($tuple:expr, value < $val:expr) => {
        if &$tuple.6 >= $val {
            panic!("assertion failed: (tuple.6 < val)\n    val: {:?}\ntuple.6: {:?}\n",
                $val,
                $tuple.6,
            );
        }
    };
    ($tuple:expr, value <= $val:expr) => {
        if &$tuple.6 > $val {
            panic!("assertion failed: (tuple.6 <= val)\n    val: {:?}\ntuple.6: {:?}\n",
                $val,
                $tuple.6,
            );
        }
    };
    ($tuple:expr, value > $val:expr) => {
        if &$tuple.6 <= $val {
            panic!("assertion failed: (tuple.6 > val)\n    val: {:?}\ntuple.6: {:?}\n",
                $val,
                $tuple.6,
            );
        }
    };
    ($tuple:expr, value >= $val:expr) => {
        if &$tuple.6 < $val {
            panic!("assertion failed: (tuple.6 >= val)\n    val: {:?}\ntuple.6: {:?}\n",
                $val,
                $tuple.6,
            );
        }
    };
}

/// Asserts that the 7th `item` in a `tuple` has a relationship to some value.
/// 
/// ### Parameters
/// 
/// - `&tuple` A reference to a tuple.
/// - `&val` A reference to a value to compare to the 7th item.
/// 
/// ### Dependencies
/// 
/// - All content must implement [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
/// - `val` must implement PartialEq for the 7th type in the tuple to use `==` or `!=`.
/// - `val` must implement PartialOrd for the 7th type in the tuple to use `<`, `<=`, `>`, `>=`.
/// 
/// ### Example
///
/// ```
/// use totems::assert_7th;
/// let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
/// let x = 8;
/// assert_7th!(&tuple, value == &x); // tuple.7 == x
/// assert_7th!(&tuple, value <= &x);
/// assert_7th!(&tuple, value >= &x);
/// assert_7th!(&tuple, value < &(x + 1));
/// assert_7th!(&tuple, value > &(x - 1));
/// ```
///
/// ### Example Error Messages 
///
/// ```text 
/// thread 'tuples::_07th::le_correct' panicked at 'assertion failed: (tuple.7 <= val)
///     val: 0
/// tuple.7: 1
/// ', src/tuples.rs:2162:9
/// ```
#[macro_export]
macro_rules! assert_7th {
    ($tuple:expr, value == $val:expr) => {
        if &$tuple.7 != $val {
            panic!("assertion failed: (tuple.7 == val)\n    val: {:?}\ntuple.7: {:?}\n",
                $val,
                $tuple.7,
            );
        }
    };
    ($tuple:expr, value != $val:expr) => {
        if &$tuple.7 == $val {
            panic!("assertion failed: (tuple.7 != val)\n    val: {:?}\ntuple.7: {:?}\n",
                $val,
                $tuple.7,
            );
        }
    };
    ($tuple:expr, value < $val:expr) => {
        if &$tuple.7 >= $val {
            panic!("assertion failed: (tuple.7 < val)\n    val: {:?}\ntuple.7: {:?}\n",
                $val,
                $tuple.7,
            );
        }
    };
    ($tuple:expr, value <= $val:expr) => {
        if &$tuple.7 > $val {
            panic!("assertion failed: (tuple.7 <= val)\n    val: {:?}\ntuple.7: {:?}\n",
                $val,
                $tuple.7,
            );
        }
    };
    ($tuple:expr, value > $val:expr) => {
        if &$tuple.7 <= $val {
            panic!("assertion failed: (tuple.7 > val)\n    val: {:?}\ntuple.7: {:?}\n",
                $val,
                $tuple.7,
            );
        }
    };
    ($tuple:expr, value >= $val:expr) => {
        if &$tuple.7 < $val {
            panic!("assertion failed: (tuple.7 >= val)\n    val: {:?}\ntuple.7: {:?}\n",
                $val,
                $tuple.7,
            );
        }
    };
}

/// Asserts that the 8th `item` in a `tuple` has a relationship to some value.
/// 
/// ### Parameters
/// 
/// - `&tuple` A reference to a tuple.
/// - `&val` A reference to a value to compare to the 8th item.
/// 
/// ### Dependencies
/// 
/// - All content must implement [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
/// - `val` must implement PartialEq for the 8th type in the tuple to use `==` or `!=`.
/// - `val` must implement PartialOrd for the 8th type in the tuple to use `<`, `<=`, `>`, `>=`.
/// 
/// ### Example
///
/// ```
/// use totems::assert_8th;
/// let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
/// let x = 9;
/// assert_8th!(&tuple, value == &x); // tuple.8 == x
/// assert_8th!(&tuple, value <= &x);
/// assert_8th!(&tuple, value >= &x);
/// assert_8th!(&tuple, value < &(x + 1));
/// assert_8th!(&tuple, value > &(x - 1));
/// ```
///
/// ### Example Error Messages 
///
/// ```text 
/// thread 'tuples::_08th::le_correct' panicked at 'assertion failed: (tuple.8 <= val)
///     val: 0
/// tuple.8: 1
/// ', src/tuples.rs:2162:9
/// ```
#[macro_export]
macro_rules! assert_8th {
    ($tuple:expr, value == $val:expr) => {
        if &$tuple.8 != $val {
            panic!("assertion failed: (tuple.8 == val)\n    val: {:?}\ntuple.8: {:?}\n",
                $val,
                $tuple.8,
            );
        }
    };
    ($tuple:expr, value != $val:expr) => {
        if &$tuple.8 == $val {
            panic!("assertion failed: (tuple.8 != val)\n    val: {:?}\ntuple.8: {:?}\n",
                $val,
                $tuple.8,
            );
        }
    };
    ($tuple:expr, value < $val:expr) => {
        if &$tuple.8 >= $val {
            panic!("assertion failed: (tuple.8 < val)\n    val: {:?}\ntuple.8: {:?}\n",
                $val,
                $tuple.8,
            );
        }
    };
    ($tuple:expr, value <= $val:expr) => {
        if &$tuple.8 > $val {
            panic!("assertion failed: (tuple.8 <= val)\n    val: {:?}\ntuple.8: {:?}\n",
                $val,
                $tuple.8,
            );
        }
    };
    ($tuple:expr, value > $val:expr) => {
        if &$tuple.8 <= $val {
            panic!("assertion failed: (tuple.8 > val)\n    val: {:?}\ntuple.8: {:?}\n",
                $val,
                $tuple.8,
            );
        }
    };
    ($tuple:expr, value >= $val:expr) => {
        if &$tuple.8 < $val {
            panic!("assertion failed: (tuple.8 >= val)\n    val: {:?}\ntuple.8: {:?}\n",
                $val,
                $tuple.8,
            );
        }
    };
}

/// Asserts that the 9th `item` in a `tuple` has a relationship to some value.
/// 
/// ### Parameters
/// 
/// - `&tuple` A reference to a tuple.
/// - `&val` A reference to a value to compare to the 9th item.
/// 
/// ### Dependencies
/// 
/// - All content must implement [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
/// - `val` must implement PartialEq for the 9th type in the tuple to use `==` or `!=`.
/// - `val` must implement PartialOrd for the 9th type in the tuple to use `<`, `<=`, `>`, `>=`.
/// 
/// ### Example
///
/// ```
/// use totems::assert_9th;
/// let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
/// let x = 10;
/// assert_9th!(&tuple, value == &x); // tuple.9 == x
/// assert_9th!(&tuple, value <= &x);
/// assert_9th!(&tuple, value >= &x);
/// assert_9th!(&tuple, value < &(x + 1));
/// assert_9th!(&tuple, value > &(x - 1));
/// ```
///
/// ### Example Error Messages 
///
/// ```text 
/// thread 'tuples::_09th::le_correct' panicked at 'assertion failed: (tuple.9 <= val)
///     val: 0
/// tuple.9: 1
/// ', src/tuples.rs:2162:9
/// ```
#[macro_export]
macro_rules! assert_9th {
    ($tuple:expr, value == $val:expr) => {
        if &$tuple.9 != $val {
            panic!("assertion failed: (tuple.9 == val)\n    val: {:?}\ntuple.9: {:?}\n",
                $val,
                $tuple.9,
            );
        }
    };
    ($tuple:expr, value != $val:expr) => {
        if &$tuple.9 == $val {
            panic!("assertion failed: (tuple.9 != val)\n    val: {:?}\ntuple.9: {:?}\n",
                $val,
                $tuple.9,
            );
        }
    };
    ($tuple:expr, value < $val:expr) => {
        if &$tuple.9 >= $val {
            panic!("assertion failed: (tuple.9 < val)\n    val: {:?}\ntuple.9: {:?}\n",
                $val,
                $tuple.9,
            );
        }
    };
    ($tuple:expr, value <= $val:expr) => {
        if &$tuple.9 > $val {
            panic!("assertion failed: (tuple.9 <= val)\n    val: {:?}\ntuple.9: {:?}\n",
                $val,
                $tuple.9,
            );
        }
    };
    ($tuple:expr, value > $val:expr) => {
        if &$tuple.9 <= $val {
            panic!("assertion failed: (tuple.9 > val)\n    val: {:?}\ntuple.9: {:?}\n",
                $val,
                $tuple.9,
            );
        }
    };
    ($tuple:expr, value >= $val:expr) => {
        if &$tuple.9 < $val {
            panic!("assertion failed: (tuple.9 >= val)\n    val: {:?}\ntuple.9: {:?}\n",
                $val,
                $tuple.9,
            );
        }
    };
}

/// Asserts that the 10th `item` in a `tuple` has a relationship to some value.
/// 
/// ### Parameters
/// 
/// - `&tuple` A reference to a tuple.
/// - `&val` A reference to a value to compare to the 10th item.
/// 
/// ### Dependencies
/// 
/// - All content must implement [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
/// - `val` must implement PartialEq for the 10th type in the tuple to use `==` or `!=`.
/// - `val` must implement PartialOrd for the 10th type in the tuple to use `<`, `<=`, `>`, `>=`.
/// 
/// ### Example
///
/// ```
/// use totems::assert_10th;
/// let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
/// let x = 11;
/// assert_10th!(&tuple, value == &x); // tuple.10 == x
/// assert_10th!(&tuple, value <= &x);
/// assert_10th!(&tuple, value >= &x);
/// assert_10th!(&tuple, value < &(x + 1));
/// assert_10th!(&tuple, value > &(x - 1));
/// ```
///
/// ### Example Error Messages 
///
/// ```text 
/// thread 'tuples::_010th::le_correct' panicked at 'assertion failed: (tuple.10 <= val)
///      val: 0
/// tuple.10: 1
/// ', src/tuples.rs:2162:9
/// ```
#[macro_export]
macro_rules! assert_10th {
    ($tuple:expr, value == $val:expr) => {
        if &$tuple.10 != $val {
            panic!("assertion failed: (tuple.10 == val)\n     val: {:?}\ntuple.10: {:?}\n",
                $val,
                $tuple.10,
            );
        }
    };
    ($tuple:expr, value != $val:expr) => {
        if &$tuple.10 == $val {
            panic!("assertion failed: (tuple.10 != val)\n     val: {:?}\ntuple.10: {:?}\n",
                $val,
                $tuple.10,
            );
        }
    };
    ($tuple:expr, value < $val:expr) => {
        if &$tuple.10 >= $val {
            panic!("assertion failed: (tuple.10 < val)\n     val: {:?}\ntuple.10: {:?}\n",
                $val,
                $tuple.10,
            );
        }
    };
    ($tuple:expr, value <= $val:expr) => {
        if &$tuple.10 > $val {
            panic!("assertion failed: (tuple.10 <= val)\n     val: {:?}\ntuple.10: {:?}\n",
                $val,
                $tuple.10,
            );
        }
    };
    ($tuple:expr, value > $val:expr) => {
        if &$tuple.10 <= $val {
            panic!("assertion failed: (tuple.10 > val)\n     val: {:?}\ntuple.10: {:?}\n",
                $val,
                $tuple.10,
            );
        }
    };
    ($tuple:expr, value >= $val:expr) => {
        if &$tuple.10 < $val {
            panic!("assertion failed: (tuple.10 >= val)\n     val: {:?}\ntuple.10: {:?}\n",
                $val,
                $tuple.10,
            );
        }
    };
}

/// Asserts that the 11th `item` in a `tuple` has a relationship to some value.
/// 
/// ### Parameters
/// 
/// - `&tuple` A reference to a tuple.
/// - `&val` A reference to a value to compare to the 11th item.
/// 
/// ### Dependencies
/// 
/// - All content must implement [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
/// - `val` must implement PartialEq for the 11th type in the tuple to use `==` or `!=`.
/// - `val` must implement PartialOrd for the 11th type in the tuple to use `<`, `<=`, `>`, `>=`.
/// 
/// ### Example
///
/// ```
/// use totems::assert_11th;
/// let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
/// let x = 12;
/// assert_11th!(&tuple, value == &x); // tuple.11 == x
/// assert_11th!(&tuple, value <= &x);
/// assert_11th!(&tuple, value >= &x);
/// assert_11th!(&tuple, value < &(x + 1));
/// assert_11th!(&tuple, value > &(x - 1));
/// ```
///
/// ### Example Error Messages 
///
/// ```text 
/// thread 'tuples::_011th::le_correct' panicked at 'assertion failed: (tuple.11 <= val)
///      val: 0
/// tuple.11: 1
/// ', src/tuples.rs:2162:9
/// ```
#[macro_export]
macro_rules! assert_11th {
    ($tuple:expr, value == $val:expr) => {
        if &$tuple.11 != $val {
            panic!("assertion failed: (tuple.11 == val)\n     val: {:?}\ntuple.11: {:?}\n",
                $val,
                $tuple.11,
            );
        }
    };
    ($tuple:expr, value != $val:expr) => {
        if &$tuple.11 == $val {
            panic!("assertion failed: (tuple.11 != val)\n     val: {:?}\ntuple.11: {:?}\n",
                $val,
                $tuple.11,
            );
        }
    };
    ($tuple:expr, value < $val:expr) => {
        if &$tuple.11 >= $val {
            panic!("assertion failed: (tuple.11 < val)\n     val: {:?}\ntuple.11: {:?}\n",
                $val,
                $tuple.11,
            );
        }
    };
    ($tuple:expr, value <= $val:expr) => {
        if &$tuple.11 > $val {
            panic!("assertion failed: (tuple.11 <= val)\n     val: {:?}\ntuple.11: {:?}\n",
                $val,
                $tuple.11,
            );
        }
    };
    ($tuple:expr, value > $val:expr) => {
        if &$tuple.11 <= $val {
            panic!("assertion failed: (tuple.11 > val)\n     val: {:?}\ntuple.11: {:?}\n",
                $val,
                $tuple.11,
            );
        }
    };
    ($tuple:expr, value >= $val:expr) => {
        if &$tuple.11 < $val {
            panic!("assertion failed: (tuple.11 >= val)\n     val: {:?}\ntuple.11: {:?}\n",
                $val,
                $tuple.11,
            );
        }
    };
}

/// Asserts that the 12th `item` in a `tuple` has a relationship to some value.
/// 
/// ### Parameters
/// 
/// - `&tuple` A reference to a tuple.
/// - `&val` A reference to a value to compare to the 12th item.
/// 
/// ### Dependencies
/// 
/// - All content must implement [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
/// - `val` must implement PartialEq for the 12th type in the tuple to use `==` or `!=`.
/// - `val` must implement PartialOrd for the 12th type in the tuple to use `<`, `<=`, `>`, `>=`.
/// 
/// ### Example
///
/// ```
/// use totems::assert_12th;
/// let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
/// let x = 13;
/// assert_12th!(&tuple, value == &x); // tuple.12 == x
/// assert_12th!(&tuple, value <= &x);
/// assert_12th!(&tuple, value >= &x);
/// assert_12th!(&tuple, value < &(x + 1));
/// assert_12th!(&tuple, value > &(x - 1));
/// ```
///
/// ### Example Error Messages 
///
/// ```text 
/// thread 'tuples::_012th::le_correct' panicked at 'assertion failed: (tuple.12 <= val)
///      val: 0
/// tuple.12: 1
/// ', src/tuples.rs:2162:9
/// ```
#[macro_export]
macro_rules! assert_12th {
    ($tuple:expr, value == $val:expr) => {
        if &$tuple.12 != $val {
            panic!("assertion failed: (tuple.12 == val)\n     val: {:?}\ntuple.12: {:?}\n",
                $val,
                $tuple.12,
            );
        }
    };
    ($tuple:expr, value != $val:expr) => {
        if &$tuple.12 == $val {
            panic!("assertion failed: (tuple.12 != val)\n     val: {:?}\ntuple.12: {:?}\n",
                $val,
                $tuple.12,
            );
        }
    };
    ($tuple:expr, value < $val:expr) => {
        if &$tuple.12 >= $val {
            panic!("assertion failed: (tuple.12 < val)\n     val: {:?}\ntuple.12: {:?}\n",
                $val,
                $tuple.12,
            );
        }
    };
    ($tuple:expr, value <= $val:expr) => {
        if &$tuple.12 > $val {
            panic!("assertion failed: (tuple.12 <= val)\n     val: {:?}\ntuple.12: {:?}\n",
                $val,
                $tuple.12,
            );
        }
    };
    ($tuple:expr, value > $val:expr) => {
        if &$tuple.12 <= $val {
            panic!("assertion failed: (tuple.12 > val)\n     val: {:?}\ntuple.12: {:?}\n",
                $val,
                $tuple.12,
            );
        }
    };
    ($tuple:expr, value >= $val:expr) => {
        if &$tuple.12 < $val {
            panic!("assertion failed: (tuple.12 >= val)\n     val: {:?}\ntuple.12: {:?}\n",
                $val,
                $tuple.12,
            );
        }
    };
}

/// Asserts that the 13th `item` in a `tuple` has a relationship to some value.
/// 
/// ### Parameters
/// 
/// - `&tuple` A reference to a tuple.
/// - `&val` A reference to a value to compare to the 13th item.
/// 
/// ### Dependencies
/// 
/// - All content must implement [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
/// - `val` must implement PartialEq for the 13th type in the tuple to use `==` or `!=`.
/// - `val` must implement PartialOrd for the 13th type in the tuple to use `<`, `<=`, `>`, `>=`.
/// 
/// ### Example
///
/// ```
/// use totems::assert_13th;
/// let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
/// let x = 14;
/// assert_13th!(&tuple, value == &x); // tuple.13 == x
/// assert_13th!(&tuple, value <= &x);
/// assert_13th!(&tuple, value >= &x);
/// assert_13th!(&tuple, value < &(x + 1));
/// assert_13th!(&tuple, value > &(x - 1));
/// ```
///
/// ### Example Error Messages 
///
/// ```text 
/// thread 'tuples::_013th::le_correct' panicked at 'assertion failed: (tuple.13 <= val)
///      val: 0
/// tuple.13: 1
/// ', src/tuples.rs:2162:9
/// ```
#[macro_export]
macro_rules! assert_13th {
    ($tuple:expr, value == $val:expr) => {
        if &$tuple.13 != $val {
            panic!("assertion failed: (tuple.13 == val)\n     val: {:?}\ntuple.13: {:?}\n",
                $val,
                $tuple.13,
            );
        }
    };
    ($tuple:expr, value != $val:expr) => {
        if &$tuple.13 == $val {
            panic!("assertion failed: (tuple.13 != val)\n     val: {:?}\ntuple.13: {:?}\n",
                $val,
                $tuple.13,
            );
        }
    };
    ($tuple:expr, value < $val:expr) => {
        if &$tuple.13 >= $val {
            panic!("assertion failed: (tuple.13 < val)\n     val: {:?}\ntuple.13: {:?}\n",
                $val,
                $tuple.13,
            );
        }
    };
    ($tuple:expr, value <= $val:expr) => {
        if &$tuple.13 > $val {
            panic!("assertion failed: (tuple.13 <= val)\n     val: {:?}\ntuple.13: {:?}\n",
                $val,
                $tuple.13,
            );
        }
    };
    ($tuple:expr, value > $val:expr) => {
        if &$tuple.13 <= $val {
            panic!("assertion failed: (tuple.13 > val)\n     val: {:?}\ntuple.13: {:?}\n",
                $val,
                $tuple.13,
            );
        }
    };
    ($tuple:expr, value >= $val:expr) => {
        if &$tuple.13 < $val {
            panic!("assertion failed: (tuple.13 >= val)\n     val: {:?}\ntuple.13: {:?}\n",
                $val,
                $tuple.13,
            );
        }
    };
}

/// Asserts that the 14th `item` in a `tuple` has a relationship to some value.
/// 
/// ### Parameters
/// 
/// - `&tuple` A reference to a tuple.
/// - `&val` A reference to a value to compare to the 14th item.
/// 
/// ### Dependencies
/// 
/// - All content must implement [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
/// - `val` must implement PartialEq for the 14th type in the tuple to use `==` or `!=`.
/// - `val` must implement PartialOrd for the 14th type in the tuple to use `<`, `<=`, `>`, `>=`.
/// 
/// ### Example
///
/// ```
/// use totems::assert_14th;
/// let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
/// let x = 15;
/// assert_14th!(&tuple, value == &x); // tuple.14 == x
/// assert_14th!(&tuple, value <= &x);
/// assert_14th!(&tuple, value >= &x);
/// assert_14th!(&tuple, value < &(x + 1));
/// assert_14th!(&tuple, value > &(x - 1));
/// ```
///
/// ### Example Error Messages 
///
/// ```text 
/// thread 'tuples::_014th::le_correct' panicked at 'assertion failed: (tuple.14 <= val)
///      val: 0
/// tuple.14: 1
/// ', src/tuples.rs:2162:9
/// ```
#[macro_export]
macro_rules! assert_14th {
    ($tuple:expr, value == $val:expr) => {
        if &$tuple.14 != $val {
            panic!("assertion failed: (tuple.14 == val)\n     val: {:?}\ntuple.14: {:?}\n",
                $val,
                $tuple.14,
            );
        }
    };
    ($tuple:expr, value != $val:expr) => {
        if &$tuple.14 == $val {
            panic!("assertion failed: (tuple.14 != val)\n     val: {:?}\ntuple.14: {:?}\n",
                $val,
                $tuple.14,
            );
        }
    };
    ($tuple:expr, value < $val:expr) => {
        if &$tuple.14 >= $val {
            panic!("assertion failed: (tuple.14 < val)\n     val: {:?}\ntuple.14: {:?}\n",
                $val,
                $tuple.14,
            );
        }
    };
    ($tuple:expr, value <= $val:expr) => {
        if &$tuple.14 > $val {
            panic!("assertion failed: (tuple.14 <= val)\n     val: {:?}\ntuple.14: {:?}\n",
                $val,
                $tuple.14,
            );
        }
    };
    ($tuple:expr, value > $val:expr) => {
        if &$tuple.14 <= $val {
            panic!("assertion failed: (tuple.14 > val)\n     val: {:?}\ntuple.14: {:?}\n",
                $val,
                $tuple.14,
            );
        }
    };
    ($tuple:expr, value >= $val:expr) => {
        if &$tuple.14 < $val {
            panic!("assertion failed: (tuple.14 >= val)\n     val: {:?}\ntuple.14: {:?}\n",
                $val,
                $tuple.14,
            );
        }
    };
}

/// Asserts that the 15th `item` in a `tuple` has a relationship to some value.
/// 
/// ### Parameters
/// 
/// - `&tuple` A reference to a tuple.
/// - `&val` A reference to a value to compare to the 15th item.
/// 
/// ### Dependencies
/// 
/// - All content must implement [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
/// - `val` must implement PartialEq for the 15th type in the tuple to use `==` or `!=`.
/// - `val` must implement PartialOrd for the 15th type in the tuple to use `<`, `<=`, `>`, `>=`.
/// 
/// ### Example
///
/// ```
/// use totems::assert_15th;
/// let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
/// let x = 16;
/// assert_15th!(&tuple, value == &x); // tuple.15 == x
/// assert_15th!(&tuple, value <= &x);
/// assert_15th!(&tuple, value >= &x);
/// assert_15th!(&tuple, value < &(x + 1));
/// assert_15th!(&tuple, value > &(x - 1));
/// ```
///
/// ### Example Error Messages 
///
/// ```text 
/// thread 'tuples::_015th::le_correct' panicked at 'assertion failed: (tuple.15 <= val)
///      val: 0
/// tuple.15: 1
/// ', src/tuples.rs:2162:9
/// ```
#[macro_export]
macro_rules! assert_15th {
    ($tuple:expr, value == $val:expr) => {
        if &$tuple.15 != $val {
            panic!("assertion failed: (tuple.15 == val)\n     val: {:?}\ntuple.15: {:?}\n",
                $val,
                $tuple.15,
            );
        }
    };
    ($tuple:expr, value != $val:expr) => {
        if &$tuple.15 == $val {
            panic!("assertion failed: (tuple.15 != val)\n     val: {:?}\ntuple.15: {:?}\n",
                $val,
                $tuple.15,
            );
        }
    };
    ($tuple:expr, value < $val:expr) => {
        if &$tuple.15 >= $val {
            panic!("assertion failed: (tuple.15 < val)\n     val: {:?}\ntuple.15: {:?}\n",
                $val,
                $tuple.15,
            );
        }
    };
    ($tuple:expr, value <= $val:expr) => {
        if &$tuple.15 > $val {
            panic!("assertion failed: (tuple.15 <= val)\n     val: {:?}\ntuple.15: {:?}\n",
                $val,
                $tuple.15,
            );
        }
    };
    ($tuple:expr, value > $val:expr) => {
        if &$tuple.15 <= $val {
            panic!("assertion failed: (tuple.15 > val)\n     val: {:?}\ntuple.15: {:?}\n",
                $val,
                $tuple.15,
            );
        }
    };
    ($tuple:expr, value >= $val:expr) => {
        if &$tuple.15 < $val {
            panic!("assertion failed: (tuple.15 >= val)\n     val: {:?}\ntuple.15: {:?}\n",
                $val,
                $tuple.15,
            );
        }
    };
}

#[cfg(test)]
mod _15th {
    #[test]
    fn eq_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_15th!(&tuple, value == &16);
    }

    #[test]
    #[should_panic]
    fn eq_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_15th!(&tuple, value == &15);
    }

    #[test]
    fn ne_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_15th!(&tuple, value != &15);
    }

    #[test]
    #[should_panic]
    fn ne_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_15th!(&tuple, value != &16);
    }

    #[test]
    fn lt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_15th!(&tuple, value < &17);
    }

    #[test]
    #[should_panic]
    fn lt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_15th!(&tuple, value < &16);
    }

    #[test]
    fn le_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_15th!(&tuple, value <= &16);
        assert_15th!(&tuple, value <= &17);
    }

    #[test]
    #[should_panic]
    fn le_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_15th!(&tuple, value <= &15);
    }

    #[test]
    fn gt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_15th!(&tuple, value > &15);
    }

    #[test]
    #[should_panic]
    fn gt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_15th!(&tuple, value > &16);
    }

    #[test]
    fn ge_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_15th!(&tuple, value >= &16);
        assert_15th!(&tuple, value >= &15);
    }

    #[test]
    #[should_panic]
    fn ge_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_15th!(&tuple, value >= &17);
    }
}

#[cfg(test)]
mod _14th {
    #[test]
    fn eq_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_14th!(&tuple, value == &15);
    }

    #[test]
    #[should_panic]
    fn eq_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_14th!(&tuple, value == &14);
    }

    #[test]
    fn ne_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_14th!(&tuple, value != &14);
    }

    #[test]
    #[should_panic]
    fn ne_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_14th!(&tuple, value != &15);
    }

    #[test]
    fn lt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_14th!(&tuple, value < &16);
    }

    #[test]
    #[should_panic]
    fn lt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_14th!(&tuple, value < &15);
    }

    #[test]
    fn le_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_14th!(&tuple, value <= &15);
        assert_14th!(&tuple, value <= &16);
    }

    #[test]
    #[should_panic]
    fn le_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_14th!(&tuple, value <= &14);
    }

    #[test]
    fn gt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_14th!(&tuple, value > &14);
    }

    #[test]
    #[should_panic]
    fn gt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_14th!(&tuple, value > &15);
    }

    #[test]
    fn ge_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_14th!(&tuple, value >= &15);
        assert_14th!(&tuple, value >= &14);
    }

    #[test]
    #[should_panic]
    fn ge_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_14th!(&tuple, value >= &16);
    }
}

#[cfg(test)]
mod _13th {
    #[test]
    fn eq_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_13th!(&tuple, value == &14);
    }

    #[test]
    #[should_panic]
    fn eq_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_13th!(&tuple, value == &13);
    }

    #[test]
    fn ne_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_13th!(&tuple, value != &13);
    }

    #[test]
    #[should_panic]
    fn ne_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_13th!(&tuple, value != &14);
    }

    #[test]
    fn lt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_13th!(&tuple, value < &15);
    }

    #[test]
    #[should_panic]
    fn lt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_13th!(&tuple, value < &14);
    }

    #[test]
    fn le_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_13th!(&tuple, value <= &14);
        assert_13th!(&tuple, value <= &15);
    }

    #[test]
    #[should_panic]
    fn le_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_13th!(&tuple, value <= &13);
    }

    #[test]
    fn gt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_13th!(&tuple, value > &13);
    }

    #[test]
    #[should_panic]
    fn gt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_13th!(&tuple, value > &14);
    }

    #[test]
    fn ge_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_13th!(&tuple, value >= &14);
        assert_13th!(&tuple, value >= &13);
    }

    #[test]
    #[should_panic]
    fn ge_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_13th!(&tuple, value >= &15);
    }
}

#[cfg(test)]
mod _12th {
    #[test]
    fn eq_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_12th!(&tuple, value == &13);
    }

    #[test]
    #[should_panic]
    fn eq_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_12th!(&tuple, value == &12);
    }

    #[test]
    fn ne_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_12th!(&tuple, value != &12);
    }

    #[test]
    #[should_panic]
    fn ne_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_12th!(&tuple, value != &13);
    }

    #[test]
    fn lt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_12th!(&tuple, value < &14);
    }

    #[test]
    #[should_panic]
    fn lt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_12th!(&tuple, value < &13);
    }

    #[test]
    fn le_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_12th!(&tuple, value <= &13);
        assert_12th!(&tuple, value <= &14);
    }

    #[test]
    #[should_panic]
    fn le_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_12th!(&tuple, value <= &12);
    }

    #[test]
    fn gt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_12th!(&tuple, value > &12);
    }

    #[test]
    #[should_panic]
    fn gt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_12th!(&tuple, value > &13);
    }

    #[test]
    fn ge_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_12th!(&tuple, value >= &13);
        assert_12th!(&tuple, value >= &12);
    }

    #[test]
    #[should_panic]
    fn ge_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_12th!(&tuple, value >= &14);
    }
}

#[cfg(test)]
mod _11th {
    #[test]
    fn eq_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_11th!(&tuple, value == &12);
    }

    #[test]
    #[should_panic]
    fn eq_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_11th!(&tuple, value == &11);
    }

    #[test]
    fn ne_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_11th!(&tuple, value != &11);
    }

    #[test]
    #[should_panic]
    fn ne_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_11th!(&tuple, value != &12);
    }

    #[test]
    fn lt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_11th!(&tuple, value < &13);
    }

    #[test]
    #[should_panic]
    fn lt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_11th!(&tuple, value < &12);
    }

    #[test]
    fn le_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_11th!(&tuple, value <= &12);
        assert_11th!(&tuple, value <= &13);
    }

    #[test]
    #[should_panic]
    fn le_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_11th!(&tuple, value <= &11);
    }

    #[test]
    fn gt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_11th!(&tuple, value > &11);
    }

    #[test]
    #[should_panic]
    fn gt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_11th!(&tuple, value > &12);
    }

    #[test]
    fn ge_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_11th!(&tuple, value >= &12);
        assert_11th!(&tuple, value >= &11);
    }

    #[test]
    #[should_panic]
    fn ge_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_11th!(&tuple, value >= &13);
    }
}

#[cfg(test)]
mod _10th {
    #[test]
    fn eq_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_10th!(&tuple, value == &11);
    }

    #[test]
    #[should_panic]
    fn eq_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_10th!(&tuple, value == &10);
    }

    #[test]
    fn ne_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_10th!(&tuple, value != &10);
    }

    #[test]
    #[should_panic]
    fn ne_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_10th!(&tuple, value != &11);
    }

    #[test]
    fn lt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_10th!(&tuple, value < &12);
    }

    #[test]
    #[should_panic]
    fn lt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_10th!(&tuple, value < &11);
    }

    #[test]
    fn le_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_10th!(&tuple, value <= &11);
        assert_10th!(&tuple, value <= &12);
    }

    #[test]
    #[should_panic]
    fn le_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_10th!(&tuple, value <= &10);
    }

    #[test]
    fn gt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_10th!(&tuple, value > &10);
    }

    #[test]
    #[should_panic]
    fn gt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_10th!(&tuple, value > &11);
    }

    #[test]
    fn ge_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_10th!(&tuple, value >= &11);
        assert_10th!(&tuple, value >= &10);
    }

    #[test]
    #[should_panic]
    fn ge_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_10th!(&tuple, value >= &12);
    }
}

#[cfg(test)]
mod _09th {
    #[test]
    fn eq_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_9th!(&tuple, value == &10);
    }

    #[test]
    #[should_panic]
    fn eq_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_9th!(&tuple, value == &9);
    }

    #[test]
    fn ne_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_9th!(&tuple, value != &9);
    }

    #[test]
    #[should_panic]
    fn ne_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_9th!(&tuple, value != &10);
    }

    #[test]
    fn lt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_9th!(&tuple, value < &11);
    }

    #[test]
    #[should_panic]
    fn lt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_9th!(&tuple, value < &10);
    }

    #[test]
    fn le_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_9th!(&tuple, value <= &10);
        assert_9th!(&tuple, value <= &11);
    }

    #[test]
    #[should_panic]
    fn le_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_9th!(&tuple, value <= &9);
    }

    #[test]
    fn gt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_9th!(&tuple, value > &9);
    }

    #[test]
    #[should_panic]
    fn gt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_9th!(&tuple, value > &10);
    }

    #[test]
    fn ge_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_9th!(&tuple, value >= &10);
        assert_9th!(&tuple, value >= &9);
    }

    #[test]
    #[should_panic]
    fn ge_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_9th!(&tuple, value >= &11);
    }
}

#[cfg(test)]
mod _08th {
    #[test]
    fn eq_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_8th!(&tuple, value == &9);
    }

    #[test]
    #[should_panic]
    fn eq_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_8th!(&tuple, value == &8);
    }

    #[test]
    fn ne_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_8th!(&tuple, value != &8);
    }

    #[test]
    #[should_panic]
    fn ne_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_8th!(&tuple, value != &9);
    }

    #[test]
    fn lt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_8th!(&tuple, value < &10);
    }

    #[test]
    #[should_panic]
    fn lt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_8th!(&tuple, value < &9);
    }

    #[test]
    fn le_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_8th!(&tuple, value <= &9);
        assert_8th!(&tuple, value <= &10);
    }

    #[test]
    #[should_panic]
    fn le_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_8th!(&tuple, value <= &8);
    }

    #[test]
    fn gt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_8th!(&tuple, value > &8);
    }

    #[test]
    #[should_panic]
    fn gt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_8th!(&tuple, value > &9);
    }

    #[test]
    fn ge_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_8th!(&tuple, value >= &9);
        assert_8th!(&tuple, value >= &8);
    }

    #[test]
    #[should_panic]
    fn ge_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_8th!(&tuple, value >= &10);
    }
}

#[cfg(test)]
mod _07th {
    #[test]
    fn eq_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_7th!(&tuple, value == &8);
    }

    #[test]
    #[should_panic]
    fn eq_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_7th!(&tuple, value == &7);
    }

    #[test]
    fn ne_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_7th!(&tuple, value != &7);
    }

    #[test]
    #[should_panic]
    fn ne_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_7th!(&tuple, value != &8);
    }

    #[test]
    fn lt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_7th!(&tuple, value < &9);
    }

    #[test]
    #[should_panic]
    fn lt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_7th!(&tuple, value < &8);
    }

    #[test]
    fn le_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_7th!(&tuple, value <= &8);
        assert_7th!(&tuple, value <= &9);
    }

    #[test]
    #[should_panic]
    fn le_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_7th!(&tuple, value <= &7);
    }

    #[test]
    fn gt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_7th!(&tuple, value > &7);
    }

    #[test]
    #[should_panic]
    fn gt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_7th!(&tuple, value > &8);
    }

    #[test]
    fn ge_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_7th!(&tuple, value >= &8);
        assert_7th!(&tuple, value >= &7);
    }

    #[test]
    #[should_panic]
    fn ge_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_7th!(&tuple, value >= &9);
    }
}



#[cfg(test)]
mod _06th {
    #[test]
    fn eq_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_6th!(&tuple, value == &7);
    }

    #[test]
    #[should_panic]
    fn eq_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_6th!(&tuple, value == &6);
    }

    #[test]
    fn ne_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_6th!(&tuple, value != &6);
    }

    #[test]
    #[should_panic]
    fn ne_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_6th!(&tuple, value != &7);
    }

    #[test]
    fn lt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_6th!(&tuple, value < &8);
    }

    #[test]
    #[should_panic]
    fn lt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_6th!(&tuple, value < &7);
    }

    #[test]
    fn le_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_6th!(&tuple, value <= &7);
        assert_6th!(&tuple, value <= &8);
    }

    #[test]
    #[should_panic]
    fn le_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_6th!(&tuple, value <= &6);
    }

    #[test]
    fn gt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_6th!(&tuple, value > &6);
    }

    #[test]
    #[should_panic]
    fn gt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_6th!(&tuple, value > &7);
    }

    #[test]
    fn ge_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_6th!(&tuple, value >= &7);
        assert_6th!(&tuple, value >= &6);
    }

    #[test]
    #[should_panic]
    fn ge_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_6th!(&tuple, value >= &8);
    }
}

#[cfg(test)]
mod _05th {
    #[test]
    fn eq_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_5th!(&tuple, value == &6);
    }

    #[test]
    #[should_panic]
    fn eq_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_5th!(&tuple, value == &5);
    }

    #[test]
    fn ne_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_5th!(&tuple, value != &5);
    }

    #[test]
    #[should_panic]
    fn ne_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_5th!(&tuple, value != &6);
    }

    #[test]
    fn lt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_5th!(&tuple, value < &7);
    }

    #[test]
    #[should_panic]
    fn lt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_5th!(&tuple, value < &6);
    }

    #[test]
    fn le_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_5th!(&tuple, value <= &6);
        assert_5th!(&tuple, value <= &7);
    }

    #[test]
    #[should_panic]
    fn le_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_5th!(&tuple, value <= &5);
    }

    #[test]
    fn gt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_5th!(&tuple, value > &5);
    }

    #[test]
    #[should_panic]
    fn gt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_5th!(&tuple, value > &6);
    }

    #[test]
    fn ge_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_5th!(&tuple, value >= &6);
        assert_5th!(&tuple, value >= &5);
    }

    #[test]
    #[should_panic]
    fn ge_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_5th!(&tuple, value >= &7);
    }
}

#[cfg(test)]
mod _04th {
    #[test]
    fn eq_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_4th!(&tuple, value == &5);
    }

    #[test]
    #[should_panic]
    fn eq_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_4th!(&tuple, value == &4);
    }

    #[test]
    fn ne_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_4th!(&tuple, value != &4);
    }

    #[test]
    #[should_panic]
    fn ne_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_4th!(&tuple, value != &5);
    }

    #[test]
    fn lt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_4th!(&tuple, value < &6);
    }

    #[test]
    #[should_panic]
    fn lt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_4th!(&tuple, value < &5);
    }

    #[test]
    fn le_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_4th!(&tuple, value <= &5);
        assert_4th!(&tuple, value <= &6);
    }

    #[test]
    #[should_panic]
    fn le_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_4th!(&tuple, value <= &4);
    }

    #[test]
    fn gt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_4th!(&tuple, value > &4);
    }

    #[test]
    #[should_panic]
    fn gt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_4th!(&tuple, value > &5);
    }

    #[test]
    fn ge_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_4th!(&tuple, value >= &5);
        assert_4th!(&tuple, value >= &4);
    }

    #[test]
    #[should_panic]
    fn ge_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_4th!(&tuple, value >= &6);
    }
}

#[cfg(test)]
mod _03rd {
    #[test]
    fn eq_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_3rd!(&tuple, value == &4);
    }

    #[test]
    #[should_panic]
    fn eq_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_3rd!(&tuple, value == &3);
    }

    #[test]
    fn ne_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_3rd!(&tuple, value != &3);
    }

    #[test]
    #[should_panic]
    fn ne_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_3rd!(&tuple, value != &4);
    }

    #[test]
    fn lt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_3rd!(&tuple, value < &5);
    }

    #[test]
    #[should_panic]
    fn lt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_3rd!(&tuple, value < &4);
    }

    #[test]
    fn le_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_3rd!(&tuple, value <= &4);
        assert_3rd!(&tuple, value <= &5);
    }

    #[test]
    #[should_panic]
    fn le_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_3rd!(&tuple, value <= &3);
    }

    #[test]
    fn gt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_3rd!(&tuple, value > &3);
    }

    #[test]
    #[should_panic]
    fn gt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_3rd!(&tuple, value > &4);
    }

    #[test]
    fn ge_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_3rd!(&tuple, value >= &4);
        assert_3rd!(&tuple, value >= &3);
    }

    #[test]
    #[should_panic]
    fn ge_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_3rd!(&tuple, value >= &5);
    }
}

#[cfg(test)]
mod _02nd {
    #[test]
    fn eq_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_2nd!(&tuple, value == &3);
    }

    #[test]
    #[should_panic]
    fn eq_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_2nd!(&tuple, value == &2);
    }

    #[test]
    fn ne_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_2nd!(&tuple, value != &2);
    }

    #[test]
    #[should_panic]
    fn ne_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_2nd!(&tuple, value != &3);
    }

    #[test]
    fn lt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_2nd!(&tuple, value < &4);
    }

    #[test]
    #[should_panic]
    fn lt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_2nd!(&tuple, value < &3);
    }

    #[test]
    fn le_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_2nd!(&tuple, value <= &3);
        assert_2nd!(&tuple, value <= &4);
    }

    #[test]
    #[should_panic]
    fn le_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_2nd!(&tuple, value <= &2);
    }

    #[test]
    fn gt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_2nd!(&tuple, value > &2);
    }

    #[test]
    #[should_panic]
    fn gt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_2nd!(&tuple, value > &3);
    }

    #[test]
    fn ge_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_2nd!(&tuple, value >= &3);
        assert_2nd!(&tuple, value >= &2);
    }

    #[test]
    #[should_panic]
    fn ge_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_2nd!(&tuple, value >= &4);
    }
}

#[cfg(test)]
mod _01st {
    #[test]
    fn eq_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_1st!(&tuple, value == &2);
    }

    #[test]
    #[should_panic]
    fn eq_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_1st!(&tuple, value == &1);
    }

    #[test]
    fn ne_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_1st!(&tuple, value != &1);
    }

    #[test]
    #[should_panic]
    fn ne_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_1st!(&tuple, value != &2);
    }

    #[test]
    fn lt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_1st!(&tuple, value < &3);
    }

    #[test]
    #[should_panic]
    fn lt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_1st!(&tuple, value < &2);
    }

    #[test]
    fn le_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_1st!(&tuple, value <= &2);
        assert_1st!(&tuple, value <= &3);
    }

    #[test]
    #[should_panic]
    fn le_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_1st!(&tuple, value <= &1);
    }

    #[test]
    fn gt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_1st!(&tuple, value > &1);
    }

    #[test]
    #[should_panic]
    fn gt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_1st!(&tuple, value > &2);
    }

    #[test]
    fn ge_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_1st!(&tuple, value >= &2);
        assert_1st!(&tuple, value >= &1);
    }

    #[test]
    #[should_panic]
    fn ge_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_1st!(&tuple, value >= &3);
    }
}

#[cfg(test)]
mod _00th {
    #[test]
    fn eq_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_0th!(&tuple, value == &1);
    }

    #[test]
    #[should_panic]
    fn eq_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_0th!(&tuple, value == &6);
    }

    #[test]
    fn ne_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_0th!(&tuple, value != &2);
    }

    #[test]
    #[should_panic]
    fn ne_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_0th!(&tuple, value != &1);
    }

    #[test]
    fn lt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_0th!(&tuple, value < &2);
    }

    #[test]
    #[should_panic]
    fn lt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_0th!(&tuple, value < &1);
    }

    #[test]
    fn le_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_0th!(&tuple, value <= &1);
        assert_0th!(&tuple, value <= &2);
    }

    #[test]
    #[should_panic]
    fn le_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_0th!(&tuple, value <= &0);
    }

    #[test]
    fn gt_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_0th!(&tuple, value > &0);
    }

    #[test]
    #[should_panic]
    fn gt_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_0th!(&tuple, value > &1);
    }

    #[test]
    fn ge_correct() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_0th!(&tuple, value >= &1);
        assert_0th!(&tuple, value >= &0);
    }

    #[test]
    #[should_panic]
    fn ge_incorrect() {
        let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
        assert_0th!(&tuple, value >= &2);
    }
}