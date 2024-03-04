//! # Diagonal Operations Module
//!
//! This module provides functions to extract diagonals from a matrix in various directions.
//!
//! ## Diagonals with Positive Slope (Bottom-Left to Top-Right)
//!
//! The `diagonal_pos_pos` function extracts diagonals with positive slope from a matrix starting
//! from the bottom-left corner (x: maximum, y: 0).
//!
//! ### Example
//!
//! ```rust
//! use diagonal::diagonal_pos_pos;
//!
//! let matrix = vec![
//!     vec![1, 2, 3],
//!     vec![4, 5, 6],
//!     vec![7, 8, 9],
//! ];
//!
//! let result = diagonal_pos_pos(&matrix);
//! assert_eq!(result, vec![
//!     vec![&7],
//!     vec![&4, &8],
//!     vec![&1, &5, &9],
//!     vec![&2, &6],
//!     vec![&3],
//! ]);
//! ```
//!
//! ## Diagonals with Positive and Negative Slope (Top-Left to Bottom-Right and Bottom-Left to Top-Right)
//!
//! The `diagonal_pos_neg` function extracts diagonals with positive and negative slope from a matrix
//! starting from the top-left corner (x & y: 0).
//!
//! ### Example
//!
//! ```rust
//! use diagonal::diagonal_pos_neg;
//!
//! let matrix = vec![
//!     vec![1, 2, 3],
//!     vec![4, 5, 6],
//!     vec![7, 8, 9],
//! ];
//!
//! let result = diagonal_pos_neg(&matrix);
//! assert_eq!(result, vec![
//!     vec![&1],
//!     vec![&2, &4],
//!     vec![&3, &5, &7],
//!     vec![&6, &8],
//!     vec![&9],
//! ]);
//! ```
//!
//! ## Straight Rows and Columns
//!
//! The `straight_x` and `straight_y` functions extract elements from a matrix in row-major and
//! column-major orders, respectively.
//!
//! ### Example
//!
//! ```rust
//! use diagonal::{straight_x, straight_y};
//!
//! let matrix = vec![
//!     vec![1, 2, 3],
//!     vec![4, 5, 6],
//!     vec![7, 8, 9],
//! ];
//!
//! let result_x = straight_x(&matrix);
//! assert_eq!(result_x, vec![
//!     vec![&1, &2, &3],
//!     vec![&4, &5, &6],
//!     vec![&7, &8, &9],
//! ]);
//!
//! let result_y = straight_y(&matrix);
//! assert_eq!(result_y, vec![
//!     vec![&1, &4, &7],
//!     vec![&2, &5, &8],
//!     vec![&3, &6, &9],
//! ]);
//! ```
//!
//! The provided functions enable convenient extraction and manipulation of matrix diagonals,
//! making it easier to perform various operations on matrix elements.
//!
//! # Note
//!
//! Make sure to use the `diagonal` crate and import the necessary functions for these examples to work.
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! diagonal = "0.1.0"
//! ```

/// Extracts diagonals with positive slope from a matrix starting from the bottom-left (x: maximum, y: 0) corner.
///
/// Given a matrix, this function traverses diagonals with `x+ y+` slope starting from
/// the bottom-left (x: maximum, y: 0) corner and returns a vector of vectors containing references to the
/// elements along these diagonals.
///
/// # Arguments
///
/// * `matrix` - A reference to a matrix represented as a slice of rows, where each row
///   is a slice of generic type `T`.
///
/// # Returns
///
/// A vector of vectors containing references to the elements in diagonals with positive slope.
/// Each inner vector represents a diagonal.
///
/// # Examples
///
/// ```
/// use diagonal::diagonal_pos_pos;
///
/// let matrix = vec![
///     vec![1, 2, 3],
///     vec![4, 5, 6],
///     vec![7, 8, 9],
/// ];
///
/// let result = diagonal_pos_pos(&matrix);
/// assert_eq!(result, vec![
///     vec![&7],
///     vec![&4, &8],
///     vec![&1, &5, &9],
///     vec![&2, &6],
///     vec![&3],
/// ]);
/// ```
pub fn diagonal_pos_pos<'a, Matrix: AsRef<[Row]> + 'a, Row: AsRef<[T]> + 'a, T>(
    matrix: &'a Matrix,
) -> Vec<Vec<&'a T>> {
    let matrix = matrix.as_ref();
    let x_len = matrix.len();
    if matrix.is_empty() {
        return vec![];
    }
    let y_len = matrix[0].as_ref().len();

    let mut x = x_len - 1;
    let mut y = 0;

    let mut result: Vec<Vec<&T>> = vec![vec![]];

    loop {
        result.last_mut().unwrap().push(&matrix[x].as_ref()[y]);

        x += 1;
        y += 1;

        if !(0..x_len).contains(&x) || !(0..y_len).contains(&y) {
            y += 1;
            let min = x.min(y);
            x -= min;
            y -= min;

            result.push(Vec::new());
        }

        if !(0..x_len).contains(&x) || !(0..y_len).contains(&y) {
            break;
        }
    }

    result.pop();
    result
}

/// Extracts diagonals with positive and negative slope from a matrix starting from the top-left (x & y: 0) corner.
///
/// Given a matrix, this function traverses diagonals with the `x+ y-` slope starting from
/// the top-left corner and returns a vector of vectors containing references to the
/// elements along these diagonals.
///
/// The positive slope diagonals are traversed in the `x+ y-` direction, while the negative slope diagonals
/// are traversed in the bottom-right direction.
///
/// # Arguments
///
/// * `matrix` - A reference to a matrix represented as a slice of rows, where each row
///   is a slice of generic type `T`.
///
/// # Returns
///
/// A vector of vectors containing references to the elements in diagonals.
/// Each inner vector represents a diagonal.
///
/// # Examples
///
/// ```
/// use diagonal::diagonal_pos_neg;
///
/// let matrix = vec![
///     vec![1, 2, 3],
///     vec![4, 5, 6],
///     vec![7, 8, 9],
/// ];
///
/// let result = diagonal_pos_neg(&matrix);
/// assert_eq!(result, vec![
///     vec![&1],
///     vec![&2, &4],
///     vec![&3, &5, &7],
///     vec![&6, &8],
///     vec![&9],
/// ]);
/// ```
pub fn diagonal_pos_neg<'a, Matrix: AsRef<[Row]> + 'a, Row: AsRef<[T]> + 'a, T>(
    matrix: &'a Matrix,
) -> Vec<Vec<&'a T>> {
    let matrix = matrix.as_ref();
    let x_len = matrix.len();
    if matrix.is_empty() {
        return vec![];
    }
    let y_len = matrix[0].as_ref().len();

    if matrix.is_empty() {
        return vec![];
    }

    let mut x: isize = 0;
    let mut y: isize = 0;

    let mut result: Vec<Vec<&T>> = vec![vec![]];

    loop {
        result
            .last_mut()
            .unwrap()
            .push(&matrix[x as usize].as_ref()[y as usize]);

        x += 1;
        y -= 1;

        if !(0..x_len).contains(&(x as usize)) || !(0..y_len).contains(&(y as usize)) {
            x += 1;
            let min = x.min(y_len as isize - 1 - y);
            x -= min;
            y += min;

            result.push(Vec::new());
        }

        if !(0..x_len).contains(&(x as usize)) || !(0..y_len).contains(&(y as usize)) {
            break;
        }
    }

    result.pop();
    result
}

/// Extracts elements from a matrix in a row-major order and organizes them into vectors,
/// where each vector represents a row of the original matrix.
///
/// # Arguments
///
/// * `matrix` - A reference to a matrix (2D array) where each row contains elements of type `T`.
///
/// # Returns
///
/// A `Vec<Vec<&T>>` containing vectors of references to the elements of the input matrix,
/// organized in rows.
///
/// # Examples
///
/// ```
/// use diagonal::straight_x;
///
/// let matrix = vec![
///     vec![1, 2, 3],
///     vec![4, 5, 6],
///     vec![7, 8, 9],
/// ];
///
/// let result = straight_x(&matrix);
/// assert_eq!(result, vec![
///     vec![&1, &2, &3],
///     vec![&4, &5, &6],
///     vec![&7, &8, &9],
/// ]);
/// ```
pub fn straight_x<'a, Matrix: AsRef<[Row]> + 'a, Row: AsRef<[T]> + 'a, T>(
    matrix: &'a Matrix,
) -> Vec<Vec<&'a T>> {
    let matrix = matrix.as_ref();
    if matrix.is_empty() {
        return vec![];
    }
    let y_len = matrix[0].as_ref().len();

    let mut result: Vec<Vec<&T>> = vec![vec![]];

    for x in matrix.iter() {
        for y in 0..y_len {
            result.last_mut().unwrap().push(&x.as_ref()[y]);
        }
        result.push(Vec::new());
    }

    result.pop();
    result
}

/// Extracts elements from a matrix in a column-major order and organizes them into vectors,
/// where each vector represents a column of the original matrix.
///
/// # Arguments
///
/// * `matrix` - A reference to a matrix (2D array) where each row contains elements of type `T`.
///
/// # Returns
///
/// A `Vec<Vec<&T>>` containing vectors of references to the elements of the input matrix,
/// organized in columns.
///
/// # Examples
///
/// ```
/// use diagonal::straight_y;
///
/// let matrix = vec![
///     vec![1, 2, 3],
///     vec![4, 5, 6],
///     vec![7, 8, 9],
/// ];
///
/// let result = straight_y(&matrix);
/// assert_eq!(result, vec![
///     vec![&1, &4, &7],
///     vec![&2, &5, &8],
///     vec![&3, &6, &9],
/// ]);
/// ```
pub fn straight_y<'a, Matrix: AsRef<[Row]> + 'a, Row: AsRef<[T]> + 'a, T>(
    matrix: &'a Matrix,
) -> Vec<Vec<&'a T>> {
    let matrix = matrix.as_ref();
    if matrix.is_empty() {
        return vec![];
    }

    let mut result: Vec<Vec<&T>> = vec![vec![]];

    for y in 0..matrix[0].as_ref().len() {
        for x in matrix.iter() {
            result.last_mut().unwrap().push(&x.as_ref()[y]);
        }
        result.push(Vec::new());
    }

    result.pop();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pos_two_by_two() {
        let matrix = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(
            diagonal_pos_pos(&matrix),
            vec![vec![&3], vec![&1, &4], vec![&2]]
        );

        let matrix = [[1, 2], [3, 4]];
        assert_eq!(
            diagonal_pos_pos(&matrix),
            vec![vec![&3], vec![&1, &4], vec![&2]]
        );
    }

    #[test]
    fn pos_one_by_one() {
        let matrix = vec![vec![1]];
        assert_eq!(diagonal_pos_pos(&matrix), vec![vec![&1]]);

        let matrix = [[1]];
        assert_eq!(diagonal_pos_pos(&matrix), vec![vec![&1]]);
    }

    #[test]
    fn pos_empty() {
        let matrix: Vec<Vec<usize>> = vec![];
        assert_eq!(diagonal_pos_pos(&matrix), Vec::<Vec<&usize>>::new());

        let matrix: [[usize; 0]; 0] = [];
        assert_eq!(diagonal_pos_pos(&matrix), Vec::<Vec<&usize>>::new());
    }

    #[test]
    fn pos_two_by_three() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(
            diagonal_pos_pos(&matrix),
            vec![vec![&4], vec![&1, &5], vec![&2, &6], vec![&3]]
        );
        let matrix = [[1, 2, 3], [4, 5, 6]];
        assert_eq!(
            diagonal_pos_pos(&matrix),
            vec![vec![&4], vec![&1, &5], vec![&2, &6], vec![&3]]
        );
    }

    #[test]
    fn pos_four_by_four() {
        let matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        assert_eq!(
            diagonal_pos_pos(&matrix),
            vec![
                vec![&13],
                vec![&9, &14],
                vec![&5, &10, &15],
                vec![&1, &6, &11, &16],
                vec![&2, &7, &12],
                vec![&3, &8],
                vec![&4]
            ]
        );

        let matrix = [
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ];
        assert_eq!(
            diagonal_pos_pos(&matrix),
            vec![
                vec![&13],
                vec![&9, &14],
                vec![&5, &10, &15],
                vec![&1, &6, &11, &16],
                vec![&2, &7, &12],
                vec![&3, &8],
                vec![&4]
            ]
        );
    }

    #[test]
    fn neg_two_by_two() {
        let matrix = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(
            diagonal_pos_neg(&matrix),
            vec![vec![&1], vec![&2, &3], vec![&4]]
        );

        let matrix = [[1, 2], [3, 4]];
        assert_eq!(
            diagonal_pos_neg(&matrix),
            vec![vec![&1], vec![&2, &3], vec![&4]]
        );
    }

    #[test]
    fn neg_one_by_one() {
        let matrix = vec![vec![1]];
        assert_eq!(diagonal_pos_neg(&matrix), vec![vec![&1]]);

        let matrix = [[1]];
        assert_eq!(diagonal_pos_neg(&matrix), vec![vec![&1]]);
    }

    #[test]
    fn neg_empty() {
        let matrix: Vec<Vec<usize>> = vec![];
        assert_eq!(diagonal_pos_neg(&matrix), Vec::<Vec<&usize>>::new());

        let matrix: [[usize; 0]; 0] = [];
        assert_eq!(diagonal_pos_neg(&matrix), Vec::<Vec<&usize>>::new());
    }

    #[test]
    fn neg_two_by_three() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(
            diagonal_pos_neg(&matrix),
            vec![vec![&1], vec![&2, &4], vec![&3, &5], vec![&6]]
        );

        let matrix = [[1, 2, 3], [4, 5, 6]];
        assert_eq!(
            diagonal_pos_neg(&matrix),
            vec![vec![&1], vec![&2, &4], vec![&3, &5], vec![&6]]
        );
    }

    #[test]
    fn neg_four_by_four() {
        let matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        assert_eq!(
            diagonal_pos_neg(&matrix),
            vec![
                vec![&1],
                vec![&2, &5],
                vec![&3, &6, &9],
                vec![&4, &7, &10, &13],
                vec![&8, &11, &14],
                vec![&12, &15],
                vec![&16]
            ]
        );

        let matrix = [
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ];
        assert_eq!(
            diagonal_pos_neg(&matrix),
            vec![
                vec![&1],
                vec![&2, &5],
                vec![&3, &6, &9],
                vec![&4, &7, &10, &13],
                vec![&8, &11, &14],
                vec![&12, &15],
                vec![&16]
            ]
        );
    }

    #[test]
    fn straight_x_three_by_three() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(
            straight_x(&matrix),
            vec![vec![&1, &2, &3], vec![&4, &5, &6], vec![&7, &8, &9]]
        );

        let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        assert_eq!(
            straight_x(&matrix),
            vec![vec![&1, &2, &3], vec![&4, &5, &6], vec![&7, &8, &9]]
        );
    }

    #[test]
    fn straight_x_empty() {
        let matrix: Vec<Vec<usize>> = vec![];
        assert_eq!(straight_x(&matrix), Vec::<Vec<&usize>>::new());

        let matrix: [[usize; 0]; 0] = [];
        assert_eq!(straight_x(&matrix), Vec::<Vec<&usize>>::new());
    }

    #[test]
    fn straight_y_three_by_three() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(
            straight_y(&matrix),
            vec![vec![&1, &4, &7], vec![&2, &5, &8], vec![&3, &6, &9]]
        );

        let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        assert_eq!(
            straight_y(&matrix),
            vec![vec![&1, &4, &7], vec![&2, &5, &8], vec![&3, &6, &9]]
        );
    }

    #[test]
    fn straight_y_empty() {
        let matrix: Vec<Vec<usize>> = vec![];
        assert_eq!(straight_y(&matrix), Vec::<Vec<&usize>>::new());

        let matrix: [[usize; 0]; 0] = [];
        assert_eq!(straight_y(&matrix), Vec::<Vec<&usize>>::new());
    }
}
