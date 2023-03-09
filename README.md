# try_utils

A small collections of macros for adding try guards in rust

## Examples

### `try_utils::try_return`

```rust
use try_utils::try_return;

fn my_func1(val: Option<i32>) -> i32 {
    let val = try_return!(val, {
        // do some expensive computation here
        panic!()
    });
    val
}

assert_eq!(my_func1(Some(10)), 10); // the expensive computation will not be run

fn my_func2(val: Option<i32>) -> i32 {
    let val = try_return!(val, 1234);
    val
}
assert_eq!(my_func2(Some(10)), 10);
assert_eq!(my_func2(None), 1234);

fn my_func3(val: Option<i32>) {
    let _ = try_return!(val);
    panic!();
}
my_func3(None);
```

### `try_utils::try_continue`
```rust
use try_utils::try_continue;

'label: for _ in 0..10 {
    for _ in 0..10 {
        let _: u32 = try_continue!(None, 'label);
        panic!();
    }
    panic!();
}

for _ in 0..10 {
    let val: u32 = try_continue!(Some(10));
    assert_eq!(val, 10);
}
```

### `try_utils::try_break`
```rust
use try_utils::try_break;

'label: for _ in 0..10 {
    for _ in 0..10 {
        let _: u32 = try_break!(None, 'label);
        panic!();
    }
    panic!();
}

for _ in 0..10 {
    let val: u32 = try_break!(Some(10));
    assert_eq!(val, 10);
}
```