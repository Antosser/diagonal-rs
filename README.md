# Diagonal Operations

This Rust module provides functions to perform operations on diagonals in matrices. It includes functions for extracting diagonals with positive slope, diagonals with positive and negative slope, as well as straight rows and columns.

## Diagonals with Positive Slope (Bottom-Left to Top-Right)

The `diagonal_pos_pos` function extracts diagonals with positive slope from a matrix starting from the bottom-left corner (x: maximum, y: 0).

### Example

```rust
use diagonal::diagonal_pos_pos;

let matrix = vec![
    vec![1, 2, 3],
    vec![4, 5, 6],
    vec![7, 8, 9],
];

let result = diagonal_pos_pos(&matrix);
assert_eq!(result, vec![
    vec![&7],
    vec![&4, &8],
    vec![&1, &5, &9],
    vec![&2, &6],
    vec![&3],
]);
```

## Diagonals with Positive and Negative Slope (Top-Left to Bottom-Right and Bottom-Left to Top-Right)

The `diagonal_pos_neg` function extracts diagonals with positive and negative slope from a matrix starting from the top-left corner (x & y: 0).

### Example

```rust
use diagonal::diagonal_pos_neg;

let matrix = vec![
    vec![1, 2, 3],
    vec![4, 5, 6],
    vec![7, 8, 9],
];

let result = diagonal_pos_neg(&matrix);
assert_eq!(result, vec![
    vec![&1],
    vec![&2, &4],
    vec![&3, &5, &7],
    vec![&6, &8],
    vec![&9],
]);
```

## Straight Rows and Columns

The `straight_x` and `straight_y` functions extract elements from a matrix in row-major and column-major orders, respectively.

### Example

```rust
use diagonal::{straight_x, straight_y};

let matrix = vec![
    vec![1, 2, 3],
    vec![4, 5, 6],
    vec![7, 8, 9],
];

let result_x = straight_x(&matrix);
assert_eq!(result_x, vec![
    vec![&1, &2, &3],
    vec![&4, &5, &6],
    vec![&7, &8, &9],
]);

let result_y = straight_y(&matrix);
assert_eq!(result_y, vec![
    vec![&1, &4, &7],
    vec![&2, &5, &8],
    vec![&3, &6, &9],
]);
```

The provided functions enable convenient extraction and manipulation of matrix diagonals, making it easier to perform various operations on matrix elements.

## Getting Started

To use this module, add the `diagonal` crate to your `Cargo.toml` file:

```toml
[dependencies]
diagonal = "0.1.0"
```

Now, you can import the necessary functions and start working with matrix diagonals in Rust!
