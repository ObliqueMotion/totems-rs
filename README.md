# totems
A collection of lightweight assertion macros for unit testing in Rust.  

I started this project solely to get more practice writing macros in Rust. Other than perhaps nicer error messages, there isn't anything in here that cannot be done with one or more regular `assert!()` macros. 

That being said, I do particularly like (and actually use) the `assert_ok!()`, `assert_err!()`, and `assert_some!()` macros with comparisons to both check the state of the enum and the inner value on the same line, which is why this exists as a crate.

## Enums

### `assert_ok!()`

**Syntax:**
```rust
use totems::assert_ok;
let result = "5".parse::<u32>();
assert_ok!(&result);
```

**Equivalent to:**
```rust
assert!(result.is_ok());
```

### `assert_ok!() with comparisons`

**Syntax:**
```rust
use totems::assert_ok;
let result = "5".parse::<u32>();
assert_ok!(&result, value == &5);
assert_ok!(&result, value != &0);
assert_ok!(&result, value <  &6);
assert_ok!(&result, value <= &6);
assert_ok!(&result, value >  &4);
assert_ok!(&result, value >= &4);
```
**Equivalent to:**
```rust
assert!(result.is_ok())
assert!(result.unwrap() >= 4);
```

### `assert_err!()`

**Synax:**
```rust
use totems::assert_err;
let result = "z".parse::<u32>();
assert_err!(&result);
```

**Equivalent to:**
```rust
assert!(result.is_err());
```

### `assert_err!() with comparisons`

**Synax:**
```rust
use totems::assert_err;
let result: Result<(), u32> = Err(5);
assert_err!(&result, value == &5);
assert_err!(&result, value != &0);
assert_err!(&result, value <  &6);
assert_err!(&result, value <= &5);
assert_err!(&result, value >  &4);
assert_err!(&result, value >= &5);
```

**Equivalent to:**
```rust
assert!(result.is_err());
assert!(result.err().unwrap() >= 5);
```

### `assert_some!()`

**Synax:**
```rust
use totems::assert_some;
let option = "5".parse::<u32>().ok();
assert_some!(&option);
```

**Equivalent to:**
```rust
assert!(option.is_some());
```

### `assert_some!() with comparisons`

**Synax:**
```rust
use totems::assert_some;
let option = "5".parse::<u32>().ok();
assert_some!(&option, value == &5);
assert_some!(&option, value != &0);
assert_some!(&option, value <  &6);
assert_some!(&option, value <= &6);
assert_some!(&option, value >  &4);
assert_some!(&option, value >= &4);
```

**Equivalent to:**
```rust
assert!(option.is_some());
assert!(option.unwrap() >= 4);
```

### `assert_none!()`

**Synax:**
```rust
use totems::assert_none;
let option = "z".parse::<u32>().ok();
assert_none!(&option);
```

**Equivalent to:**
```rust
assert!(option.is_none());
```

## Tuples

### `assert_(0 to 15)th`

**Synax:**
```rust
use totems::{assert_0th, ..., assert_15th} // pseudocode
let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, "Hello");
let x = 1;
assert_0th!(&tuple, value >= &x); // tuple.0 >= x
assert_1st!(&tuple, value >= &x);
assert_2nd!(&tuple, value >= &x);
assert_3rd!(&tuple, value >= &x);
assert_4th!(&tuple, value >= &x);
assert_5th!(&tuple, value >= &x);
assert_6th!(&tuple, value >= &x);
assert_7th!(&tuple, value >= &x);
assert_8th!(&tuple, value >= &x);
assert_9th!(&tuple, value >= &x);
assert_10th!(&tuple, value >= &x);
assert_11th!(&tuple, value >= &x);
assert_12th!(&tuple, value >= &x);
assert_13th!(&tuple, value >= &x);
assert_14th!(&tuple, value >= &x);
assert_15th!(&tuple, value >= &x);
```

**Equivalent to:**
```rust
assert!(tuple.0 >= x);
...
assert!(tuple.15 >= x);
```

## Collections

### `assert_contains!()`

**Synax:**
```rust
use totems::assert_contains;
let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
let x = 5;
assert_contains!(&vec, &x);
```

**Equivalent to:**
```rust
assert!(vec.contains(&x));
```

### `assert_all!()`

**Syntax:**
```rust
use totems::assert_all;
let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
assert_all!(&vec, |&x| x > 0, "all > 0");
```

**Equivalent to:**
```rust
assert!(vec.iter().all(|&x| x > 0));
```

### `assert_any!()`

**Syntax:**
```rust
use totems::assert_any;
let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
assert_any!(&vec, |&x| x > 0, "any > 0");
```

**Equivalent to:**
```rust
assert!(vec.iter().any(|&x| x > 0));
```

### `assert_nth!()`

**Syntax:**
```rust
use totems::assert_nth;
let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
let x = 5;
assert_nth!(&vec, 2, value == &x); // vec[2] == x
assert_nth!(&vec, 2, value <= &x);
assert_nth!(&vec, 2, value >= &x);
assert_nth!(&vec, 2, value < &(x + 1));
assert_nth!(&vec, 2, value > &(x - 1));
```

**Equivalent to:**
```rust
assert!(vec[2] >= x);
```

## Inequalities

**Syntax:**
```rust
use totems::{assert_lt, assert_le, assert_gt, assert_ge};
assert_lt!(x, y) // less than
assert_le!(x, y) // less or equal

assert_gt!(x, y) // greater than
assert_ge!(x, y) // greater or equal
```

**Equivalent to:**
```rust
assert!(x <  y);
assert!(x <= y);
assert!(x >  y);
assert!(x >= y);
```