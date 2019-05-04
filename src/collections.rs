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
    use std::collections::HashMap;
    #[test]
    fn contains_item_in_vec() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        let x = 5;
        assert_contains!(&vec, &x);
    }

    #[test]
    fn contains_item_in_map() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 3);
        let pair = (&"a", &1);
        assert_contains!(&map, pair);
    }

    #[test]
    #[should_panic]
    fn does_not_contain_item() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        let x = 2;
        assert_contains!(&vec, &x);
    }
}

#[cfg(test)]
mod all {
    use std::collections::HashMap;
    #[test]
    fn all_items_match() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        assert_all!(&vec, |&x| x > 0, "all > 0");
    }

    #[test]
    #[should_panic]
    fn one_item_matches() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 3);
        map.insert("c", 5);
        map.insert("d", 7);
        let pair = (&"a", &1);
        assert_all!(&map, |x| x == pair, r#"all == ("a", 1)"#);
    }
}


#[cfg(test)]
mod any {
    use std::collections::HashMap;
    #[test]
    fn all_items_match() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        assert_any!(&vec, |&x| x > 0, "any > 0");
    }

    #[test]
    fn one_item_matches() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 3);
        map.insert("c", 5);
        map.insert("d", 7);
        let pair = (&"a", &1);
        assert_any!(&map, |x| x == pair, r#"any == ("a", 1)"#);
    }

    #[test]
    #[should_panic]
    fn no_items_match() {
        let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        assert_any!(&vec, |&x| x < 0, "x < 0");
    }
}