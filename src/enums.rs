use std::fmt::Debug;

//=============================================================================================
// Structs
//=============================================================================================

pub struct ChainOk<T>(T);
pub struct ChainSome<T>(T);

//=============================================================================================
// Implementations
//=============================================================================================

impl<T, E> ChainOk<Result<T, E>>
where
    T: Debug + PartialEq
{
    pub fn new(result: Result<T, E>) -> Self {
        Self(result)
    }

    pub fn with_value(&self, right: &T) {
        if let ChainOk(Ok(left)) = self {
            if left != right {
                panic!(
                    "assertion failed: (Ok(left) => left == right)\n  left: {:?}\n right: {:?}\n",
                    *left, right
                );
            }
        }
    }
}

impl<T> ChainSome<Option<T>>
where
    T: Debug + PartialEq
{
    pub fn new(option: Option<T>) -> Self {
        Self(option)
    }

    pub fn with_value(&self, right: &T) {
        if let ChainSome(Some(left)) = self {
            if left != right {
                panic!(
                    "assertion failed: (Some(left) => left == right)\n  left: {:?}\n right: {:?}\n",
                    left, right
                );
            }
        }
    }
}

//=============================================================================================
// Macros
//=============================================================================================

#[macro_export]
macro_rules! assert_ok {
    ($result:expr) => {{
        use super::ChainOk;
        if let Err(_) = $result {
            panic!(
                "assertion failed: ({0} == Ok(_))\n {0}: {1:?}\n",
                stringify!($result),
                $result,
            );
        }
        ChainOk::new($result)
    }};
}

#[macro_export]
macro_rules! assert_some {
    ($option:expr) => {{
        use super::ChainSome;
        if let None = $option {
            panic!(
                "assertion failed: ({0} == Some(_))\n {0}: {1:?}\n",
                stringify!($option),
                $option,
            );
        }
        ChainSome::new($option)
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
        assert_ok!(result);
    }

    #[test]
    fn correct_value() {
        let result = "5".parse::<u32>();
        assert_ok!(result).with_value(&5);
    }

    #[test]
    #[should_panic]
    fn incorrect_value() {
        let result = "5".parse::<u32>();
        assert_ok!(result).with_value(&2);
    }

    #[test]
    #[should_panic]
    fn is_err() {
        let result = "z".parse::<u32>();
        assert_ok!(result).with_value(&5);
    }
}

#[cfg(test)]
mod some {
    #[test]
    fn stand_alone() {
        let option = "5".parse::<u32>().ok();
        assert_some!(option);
    }

    #[test]
    fn correct_value() {
        let option = "5".parse::<u32>().ok();
        assert_some!(option).with_value(&5);
    }

    #[test]
    #[should_panic]
    fn incorrect_value() {
        let option = "5".parse::<u32>().ok();
        assert_some!(option).with_value(&2);
    }

    #[test]
    #[should_panic]
    fn is_none() {
        let option = "z".parse::<u32>().ok();
        assert_some!(option).with_value(&5);
    }
}
