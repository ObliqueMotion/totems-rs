//=============================================================================================
// Macros
//=============================================================================================

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

#[macro_export]
macro_rules! assert_all {
    ($collection:expr, $predicate:expr) => {
        if false == $collection.into_iter().all($predicate) {
            panic!("assertion failed: (all elements of collection match predicate) collection: {:?}",
                $collection,
            )
        }
    };
    ($collection:expr, $predicate:expr, $($arg:tt)+) => {
        if false == $collection.into_iter().all($predicate) {
            panic!("assertion failed: (all elements of collection match predicate)\n  predicate: {}\n collection: {:?}",
                format_args!($($arg)+),
                $collection,
            )
        }
    }
}

#[macro_export]
macro_rules! assert_any {
    ($collection:expr, $predicate:expr) => {
        if false == $collection.into_iter().any($predicate) {
            panic!("assertion failed: (any element of collection matches predicate) collection: {:?}",
                $collection,
            )
        }
    };
    ($collection:expr, $predicate:expr, $($arg:tt)+) => {
        if false == $collection.into_iter().any($predicate) {
            panic!("assertion failed: (any element of collection matches predicate)\n  predicate: {}\n collection: {:?}",
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
        assert_any!(&vec, |&x| x < 0, "all < 0");
    }

    #[test]
    #[should_panic]
    fn no_items_match() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        assert_any!(&vec, |&x| x < 0, "x < 0");
    }
}