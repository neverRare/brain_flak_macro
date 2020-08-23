# Brain Flak Macro

Write [Brain-Flak] code inside rust. You can pass a vector for input. The `brain_flak` macro will expand to stack manipulation.

```rust
use brain_flak_macro::brain_flak;

let mut vec: Vec<i32> = vec![20, 5];
brain_flak! { &mut vec =>
    // multiplication
    ([({}<([({}(<()>))<>](<()>))<>>)<>]){({}[()]<(({})<({}{})>)>)<>}{}{}<>{}{}{}<>
}
assert_eq!(vec![100], vec);
```

## Why

I was about to make [brainfuck macro] but it already exist. A random guy from some random discord server suggested Brain-Flak instead. It's choice of using balanced brackets make it  easy enough for just using `macro_rules` (No).

[Brain-Flak]: https://esolangs.org/wiki/Brain-Flak
[brainfuck macro]: https://crates.io/crates/brainfuck_macros
