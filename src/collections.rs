//=============================================================================================
// Macros
//=============================================================================================

/// Asserts that the nth `item` in a `collection` has a relationship to some value.
/// 
/// ### Parameters
/// 
/// - `&collection` A reference to a collection.
/// - `position` The position in the collection (starts at 0).
/// - `&val` A reference to a value to compare to the nth item.
/// 
/// ### Dependencies
/// 
/// - All content must implement [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
/// - `&collection` must implement [IntoIterator](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html).
/// - `val` must implement PartialEq for the types in `collection` to use `==` or `!=`.
/// - `val` must implement PartialOrd for the types in `collection` to use `<`, `<=`, `>`, `>=`.
/// 
/// ### Example
///
/// ```
/// use totems::assert_nth;
/// let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
/// let x = 5;
/// assert_nth!(&vec, 2, value == &x); // vec[2] == x
/// assert_nth!(&vec, 2, value <= &x);
/// assert_nth!(&vec, 2, value >= &x);
/// assert_nth!(&vec, 2, value < &(x + 1));
/// assert_nth!(&vec, 2, value > &(x - 1));
/// ```
///
/// ### Example Error Messages 
///
/// ```text 
/// thread 'collections::nth::le_correct' panicked at 'assertion failed: (collection[3] <= item)
///          item: 5
/// collection[3]: 7
/// ', src/collections.rs:388:9
/// ```
#[macro_export]
macro_rules! assert_nth {
    ($collection:expr, $position:expr, value == $val:expr) => {
        match $collection.into_iter().nth($position) {
            Some(value) => {
                if value != $val {
                    panic!("assertion failed: (collection[{0}] == item)\n         item: {1:?}\ncollection[{0}]: {2:?}\n",
                        $position,
                        $val,
                        value,
                    )
                }
            }
            None => {
                panic!("assertion failed: (collection[{0}] == item)\n collection[{0}] does not exist\n",
                    $position,
                );
            }
        }
    };
    ($collection:expr, $position:expr, value != $val:expr) => {
        match $collection.into_iter().nth($position) {
            Some(value) => {
                if value == $val {
                    panic!("assertion failed: (collection[{0}] != item)\n         item: {1:?}\ncollection[{0}]: {2:?}\n",
                        $position,
                        $val,
                        value,
                    )
                }
            }
            None => {
                panic!("assertion failed: (collection[{0}] != item)\n collection[{0}] does not exist\n",
                    $position,
                );
            }
        }
    };
    ($collection:expr, $position:expr, value < $val:expr) => {
        match $collection.into_iter().nth($position) {
            Some(value) => {
                if value >= $val {
                    panic!("assertion failed: (collection[{0}] < item)\n         item: {1:?}\ncollection[{0}]: {2:?}\n",
                        $position,
                        $val,
                        value,
                    )
                }
            }
            None => {
                panic!("assertion failed: (collection[{0}] < item)\n collection[{0}] does not exist\n",
                    $position,
                );
            }
        }
    };
    ($collection:expr, $position:expr, value <= $val:expr) => {
        match $collection.into_iter().nth($position) {
            Some(value) => {
                if value > $val {
                    panic!("assertion failed: (collection[{0}] <= item)\n         item: {1:?}\ncollection[{0}]: {2:?}\n",
                        $position,
                        $val,
                        value,
                    )
                }
            }
            None => {
                panic!("assertion failed: (collection[{0}] <= item)\n collection[{0}] does not exist\n",
                    $position,
                );
            }
        }
    };
    ($collection:expr, $position:expr, value > $val:expr) => {
        match $collection.into_iter().nth($position) {
            Some(value) => {
                if value <= $val {
                    panic!("assertion failed: (collection[{0}] > item)\n         item: {1:?}\ncollection[{0}]: {2:?}\n",
                        $position,
                        $val,
                        value,
                    )
                }
            }
            None => {
                panic!("assertion failed: (collection[{0}] > item)\n collection[{0}] does not exist\n",
                    $position,
                );
            }
        }
    };
    ($collection:expr, $position:expr, value >= $val:expr) => {
        match $collection.into_iter().nth($position) {
            Some(value) => {
                if value < $val {
                    panic!("assertion failed: (collection[{0}] >= item)\n         item: {1:?}\ncollection[{0}]: {2:?}\n",
                        $position,
                        $val,
                        value,
                    )
                }
            }
            None => {
                panic!("assertion failed: (collection[{0}] >= item)\n collection[{0}] does not exist\n",
                    $position,
                );
            }
        }
    };
}

/// Asserts that an `item` is contained within a `collection`.
/// 
/// ### Parameters
/// 
/// - `&collection` A reference to a collection.
/// - `&item` A reference to an item to compare to items in the collection.
/// 
/// ### Dependencies
/// 
/// - All content must implement [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
/// - `&collection` must implement [IntoIterator](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html).
/// - `item` must implement PartialEq for the types in `collection`.
/// 
/// ### Example
///
/// ```
/// use totems::assert_contains;
/// let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
/// let x = 5;
/// assert_contains!(&vec, &x);
/// ```
///
/// ### Example Error Messages 
///
/// ```text 
/// thread 'main' panicked at 'assertion failed: (collection contains item)
///        item: 2
///  collection: [1, 3, 5, 7, 9, 11, 13, 15, 17, 19]
/// ', src/collections.rs:149:9
/// ```
#[macro_export]
macro_rules! assert_contains {
    ($collection:expr, $item:expr) => {
        if let None = $collection.into_iter().find(|&x| x == $item) {
            panic!("assertion failed: (collection contains item)\n       item: {:?}\n collection: {:?}\n",
                    $item,
                    $collection,
            );
        }
    };
}

/// Asserts that *all* `items` in a `collection` match a `predicate`.
/// 
/// ### Parameters
/// 
/// - `&collection` A reference to a collection.
/// - `predicate` A closure or function that takes an `item` and returns a boolean.
/// - `description` ***(optional)*** A string describing the predicate.
/// 
/// ### Dependencies
/// 
/// - All content must implement [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
/// - `&collection` must implement [IntoIterator](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html).
/// 
/// ### Example
///
/// ```
/// use totems::assert_all;
/// let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
/// assert_all!(&vec, |&x| x > 0, "all > 0");
/// ```
///
/// ### Example Error Messages 
///
/// ```text 
/// thread 'main' panicked at 'assertion failed: (all elements of collection match predicate)
///   predicate: all < 0
///  collection: [-1, 3, 5, 7, 9, 11, 13, 15, 17, 19]
/// ', src/collections.rs:165:9
/// ```
#[macro_export]
macro_rules! assert_all {
    ($collection:expr, $predicate:expr) => {
        if false == $collection.into_iter().all($predicate) {
            panic!("assertion failed: (all elements of collection match predicate) collection: {:?}\n",
                $collection,
            )
        }
    };
    ($collection:expr, $predicate:expr, $($arg:tt)+) => {
        if false == $collection.into_iter().all($predicate) {
            panic!("assertion failed: (all elements of collection match predicate)\n  predicate: {}\n collection: {:?}\n",
                format_args!($($arg)+),
                $collection,
            )
        }
    }
}

/// Asserts that *any* `item` in a `collection` matches a `predicate`.
/// 
/// ### Parameters
/// 
/// - `&collection` A reference to a collection.
/// - `predicate` A closure or function that takes an `item` and returns a boolean.
/// - `description` ***(optional)*** A string describing the predicate.
/// 
/// ### Dependencies
/// 
/// - All content must implement [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
/// - `&collection` must implement [IntoIterator](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html).
/// 
/// ### Example
///
/// ```
/// use totems::assert_any;
/// let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
/// assert_any!(&vec, |&x| x > 0, "any > 0");
/// ```
///
/// ### Example Error Messages 
///
/// ```text 
/// thread 'main' panicked at 'assertion failed: (any element of collection matches predicate)
///   predicate: any < 0
///  collection: [1, 3, 5, 7, 9, 11, 13, 15, 17, 19]
/// ', src/collections.rs:188:9
/// ```
#[macro_export]
macro_rules! assert_any {
    ($collection:expr, $predicate:expr) => {
        if false == $collection.into_iter().any($predicate) {
            panic!("assertion failed: (any element of collection matches predicate) collection: {:?}\n",
                $collection,
            )
        }
    };
    ($collection:expr, $predicate:expr, $($arg:tt)+) => {
        if false == $collection.into_iter().any($predicate) {
            panic!("assertion failed: (any element of collection matches predicate)\n  predicate: {}\n collection: {:?}\n",
                format_args!($($arg)+),
                $collection,
            )
        }
    }
}

//=============================================================================================
// Unit Tests
//=============================================================================================

#[cfg(test)]
mod contains {
    #[test]
    fn contains_item() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        let x = 5;
        assert_contains!(&vec, &x);
    }

    #[test]
    #[should_panic]
    fn excludes_item() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        let x = 2;
        assert_contains!(&vec, &x);
    }
}

#[cfg(test)]
mod nth {
    #[test]
    fn eq_correct() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        let x = 5;
        assert_nth!(&vec, 2, value == &x);
    }

    #[test]
    #[should_panic]
    fn eq_incorrect() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        let x = 6;
        assert_nth!(&vec, 2, value == &x);
    }

    #[test]
    #[should_panic]
    fn eq_out_of_range() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        let x = 6;
        assert_nth!(&vec, 20, value == &x);
    }

    #[test]
    fn ne_correct() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        let x = 6;
        assert_nth!(&vec, 2, value != &x);
    }

    #[test]
    #[should_panic]
    fn ne_incorrect() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        let x = 5;
        assert_nth!(&vec, 2, value != &x);
    }

    #[test]
    #[should_panic]
    fn ne_out_of_range() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        let x = 6;
        assert_nth!(&vec, 20, value != &x);
    }

    #[test]
    fn lt_correct() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        let x = 6;
        assert_nth!(&vec, 2, value < &x);
    }

    #[test]
    #[should_panic]
    fn lt_incorrect() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        let x = 5;
        assert_nth!(&vec, 2, value < &x);
    }

    #[test]
    #[should_panic]
    fn lt_out_of_range() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        let x = 6;
        assert_nth!(&vec, 20, value < &x);
    }

    #[test]
    fn le_correct() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        let x = 5;
        assert_nth!(&vec, 2, value <= &x);
    }

    #[test]
    #[should_panic]
    fn le_incorrect() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        let x = 4;
        assert_nth!(&vec, 2, value <= &x);
    }

    #[test]
    #[should_panic]
    fn le_out_of_range() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        let x = 6;
        assert_nth!(&vec, 20, value <= &x);
    }

    #[test]
    fn gt_correct() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        let x = 4;
        assert_nth!(&vec, 2, value > &x);
    }

    #[test]
    #[should_panic]
    fn gt_incorrect() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        let x = 5;
        assert_nth!(&vec, 2, value > &x);
    }

    #[test]
    #[should_panic]
    fn gt_out_of_range() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        let x = 6;
        assert_nth!(&vec, 20, value > &x);
    }

    #[test]
    fn ge_correct() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        let x = 5;
        assert_nth!(&vec, 2, value >= &x);
    }

    #[test]
    #[should_panic]
    fn ge_incorrect() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        let x = 6;
        assert_nth!(&vec, 2, value >= &x);
    }

    #[test]
    #[should_panic]
    fn ge_out_of_range() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        let x = 6;
        assert_nth!(&vec, 20, value >= &x);
    }
}

#[cfg(test)]
mod all {
    #[test]
    fn all_items_match() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        assert_all!(&vec, |&x| x > 0, "all > 0");
    }

    #[test]
    #[should_panic]
    fn one_item_matches() {
        let vec = vec![-1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        assert_all!(&vec, |&x| x < 0, "all < 0");
    }
}


#[cfg(test)]
mod any {
    #[test]
    fn all_items_match() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        assert_any!(&vec, |&x| x > 0, "any > 0");
    }

    #[test]
    fn one_item_matches() {
        let vec = vec![-1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        assert_any!(&vec, |&x| x < 0, "any < 0");
    }

    #[test]
    #[should_panic]
    fn no_items_match() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        assert_any!(&vec, |&x| x < 0, "any < 0");
    }
}