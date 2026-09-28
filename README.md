# orx-col-card

[![orx-col-card crate](https://img.shields.io/crates/v/orx-col-card.svg)](https://crates.io/crates/orx-col-card)
[![orx-col-card crate](https://img.shields.io/crates/d/orx-col-card.svg)](https://crates.io/crates/orx-col-card)
[![orx-col-card documentation](https://docs.rs/orx-col-card/badge.svg)](https://docs.rs/orx-col-card)

Cardinality abstractions over multi-dimensional collections (`D1`, `D2`, `D3`, `D4`), supporting standard collections such as `Vec`, `VecDeque`, slices, and their nested/mixed combinations.

## Examples

```rust
use orx_col_card::*;
use std::collections::VecDeque;

// 1D Collection
let vec = vec![10, 20, 30];
assert_eq!(vec.card([]), 3);
assert_eq!(vec.try_card([]), Some(3));

// 2D Collection (Ragged / Nested)
let matrix: Vec<VecDeque<i32>> = vec![
    VecDeque::from([1, 2, 3]),
    VecDeque::from([4]),
];

assert_eq!(Card::<D2>::card(&matrix, []), 2);
assert_eq!(Card::<D2>::card(&matrix, 0), 3);
assert_eq!(Card::<D2>::card(&matrix, [1]), 1);
assert_eq!(Card::<D2>::try_card(&matrix, 2), None);

// 3D Collection (Mixed types)
let slice_0: &[i32] = &[1, 2];
let slice_1: &[i32] = &[3, 4, 5, 6];
let tensor: Vec<VecDeque<&[i32]>> = vec![
    VecDeque::from([slice_0, slice_1]),
];

assert_eq!(Card::<D3>::card(&tensor, []), 1);
assert_eq!(Card::<D3>::card(&tensor, 0), 2);
assert_eq!(Card::<D3>::card(&tensor, [0, 1]), 4);
assert_eq!(Card::<D3>::try_card(&tensor, [0, 2]), None);
```

## License

Dual-licensed under Apache 2.0 or MIT.
