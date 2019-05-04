# totems-rs
A collection of assertion macros for unit testing in Rust.  

## Enums

### `assert_ok!()`

```rust
use totems::assert_ok;
let result = "5".parse::<u32>();
assert_ok!(result);
```

### `assert_ok!().with_value()`

```rust
use totems::assert_ok;
let result = "5".parse::<u32>();
assert_ok!(result).with_value(&5);
```

### `assert_err!()`

```rust
use totems::assert_err;
let result = "z".parse::<u32>();
assert_err!(result);
```

### `assert_some!()`

```rust
use totems::assert_some;
let option = "5".parse::<u32>().ok();
assert_some!(option);
```

### `assert_some!().with_value()`

```rust
use totems::assert_some;
let option = "5".parse::<u32>().ok();
assert_some!(option).with_value(&5);
```

### `assert_none!()`

```rust
use totems::assert_none;
let option = "z".parse::<u32>().ok();
assert_none!(option);
```

## Collections

### `assert_contains!()`

```rust
use totems::assert_contains;
let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
let x = 5;
assert_contains!(&vec, &x);
```

```rust
use totems::assert_contains;
let mut map = HashMap::new();
map.insert("a", 1);
map.insert("b", 3);
let pair = (&"a", &1);
assert_contains!(&map, pair)
```

### `assert_all!()`

```rust
use totems::assert_all;
let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
assert_all!(&vec, |&x| x > 0, "all > 0");
```

### `assert_any!()`

```rust
use totems::assert_any;
let vec = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
assert_any!(&vec, |&x| x > 0, "any > 0");
```

## Inequalities

```rust
use totems::{assert_lt, assert_le, assert_gt, assert_ge};
assert_lt!(x, y) // less than
assert_le!(x, y) // less or equal

assert_gt!(x, y) // greater than
assert_ge!(x, y) // greater or equal
```
