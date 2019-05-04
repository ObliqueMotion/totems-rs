pub trait AssertContains {
    fn assert_contains(&self);
}

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
        map.insert("c", 5);
        map.insert("d", 7);
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