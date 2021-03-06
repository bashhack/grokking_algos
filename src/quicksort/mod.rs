//! Quicksort
//!
//! `quicksort` is a module that introduces not only the simple and elegant sorting algorithm
//! we call 'Quicksort' but also the general classification of algorithmic technique it belongs
//! to, known as 'divide and conquer'.
//!
//! # Context
//!
//! Building from the foundation of problem solving covered in the `recursion` module, the
//! mechanics of divide and conquer is a method of breaking down complex problems into
//! smaller, simpler ones.
//!
//! Put simply - divide and conquer algorithms are recursive by their very nature!
//!
//! We can describe the application of divide and conquer as follows:
//!
//! * Identify the base case
//! * Divide or simplify the problem until it becomes the base case
//!
//! # Examples
//!
//! Let's rework a simple sum function to illustrate the principle.
//! We can describe the probelm as such:
//!
//! ```text
//! A function `sum` takes an array,
//! storing a reference to a sum total.
//! For every item in the array,
//! we increase the count of the sum total,
//! returning total after iteration is complete.
//! ```
//!
//! In this, the iterative case, we can envision a `for` loop mechanic underpinning the
//! approach. However, in the recursive case we must apply those rules of divide and conquer
//! in order to arrive at the appropriate refactoring.
//!
//! ```text
//! A function `sum` takes an array,
//! if that array is empty return a sum total of zero.
//! If that array is not empty, the sum total is equal
//! to the first value in the array plus the sum of the rest of the array.
//! ```
//!
//! ## Exercises
//!
//! 4.1
//!
//! Write out the code for the `sum` function
//!
//! A - 4.1
//!
//! Hello world
//!
//! ---
//!
//! 4.2
//!
//! Write a recursive function to count the number of items in a list
//!
//! A - 4.2
//!
//! Hello world 2
//!
//! ---
//!
//! 4.3
//!
//! Find the maximum number in a list
//!
//! A - 4.3
//!
//! Hello world 3
//!
//! ---
//!
//! 4.4
//!
//! Remembering binary search - it's a divide and conquer algorithm, too. Can
//! you come up with the base case and recursive case for binary search?
//!
//! A - 4.4
//!
//! [recursive_binary_search](../intro_to_algos/fn.recursive_binary_search.html)
//!

use std::cmp::max;

/// A sum implementation
///
/// Given an array, evaluates its sum - returns an unsigned integer
///
/// Should be expected to have performance characteristics of `O(n)`.
///
/// # Arguments
///
/// * `list` - a list of elements (can be empty list)
///
/// # Examples
///
/// ```rust
/// let v: Vec<u64 = (0..5).collect();
/// sum(&v)
/// ```
pub fn sum(list: &[u64]) -> u64 {
    match list.is_empty() {
        true => 0,
        _ => list[0] + sum(&list[1..]),
    }
}

/// A count implementation
///
/// Given an array, evaluates the count of elements - returns an unsigned integer
///
/// Should be expected to have performance characteristics of `O(n)`.
///
/// # Arguments
///
/// * `list` - a list of elements (can be empty list)
///
/// # Examples
///
/// ```rust
/// let v: Vec<u64 = (0..5).collect();
/// count(&v)
/// ```
pub fn count<T: std::fmt::Debug>(list: &[T]) -> u64 {
    match list.is_empty() {
        true => 0,
        _ => 1 + count(&list[1..]),
    }
}

/// A max implementation
///
/// Given an array, finds its max value - returns either the max value or None.
///
/// Should be expected to have performance characteristics of `O(n)`.
///
/// # Arguments
///
/// * `list` - a list of elements (can be empty list)
/// * `index` - an index used to instantiate recursive case
///
/// # Examples
///
/// ```rust
/// let v: Vec<u64 = (0..5).collect();
/// _find_max(&v, v.len() - 1)
/// ```
pub fn _find_max<T: Ord + Copy>(list: &[T], index: usize) -> Option<&T> {
    match list.is_empty() {
        true => None,
        _ => Some(max(
            &list[0],
            _find_max(&list[1..], index.checked_sub(1).unwrap_or(list.len() - 1))
                .unwrap_or(&list[0]),
        )),
    }
}

/// A function wrapper for [_find_max](../quicksort/fn._find_max.html)
///
/// Because Rust does not have a great way to handle optional arguments -
/// with Option, Enum, and function wrappers as alternatives - this
/// function serves to encapsulate the initial call to `_find_max`
/// in order to instantiate its parameter `index`.
///
///
/// Should be expected to have performance characteristics of `O(n)`.
///
/// # Arguments
///
/// * `list` - a list of elements (can be empty list)
///
/// # Examples
///
/// ```rust
/// let v: Vec<u64 = (0..5).collect();
/// find_max(&v)
/// ```
pub fn find_max<T: Ord + Copy>(list: &[T]) -> Option<&T> {
    let index = list.len().checked_sub(1).unwrap_or(0);
    _find_max(&list, index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_empty_list_returns_zero() {
        let v: Vec<u64> = Vec::new();
        assert_eq!(sum(&v), 0)
    }

    #[test]
    fn sum_of_non_empty_list_returns_non_zero_sum_value() {
        let v: Vec<u64> = (0..5).collect();
        assert_eq!(sum(&v), 10);
    }

    #[test]
    fn sum_of_non_empty_list_returns_value_equal_to_builtin_sum() {
        let v: Vec<u64> = (0..5).collect();
        let sum_of_v = sum(&v);
        let builtin_sum_of_v = v.iter().sum();
        assert_eq!(sum_of_v, builtin_sum_of_v);
    }

    #[test]
    fn count_of_empty_list_returns_zero() {
        let v: Vec<u64> = Vec::new();
        assert_eq!(count(&v), 0)
    }

    #[test]
    fn count_of_non_empty_list_returns_non_zero_count_value() {
        let v: Vec<u64> = (0..5).collect();
        assert_eq!(count(&v), 5);
    }

    #[test]
    fn count_of_non_empty_list_returns_value_equal_to_builtin_len() {
        let v: Vec<u64> = (0..5).collect();
        let count_of_v = count(&v);
        let builtin_count_of_v = v.len() as u64;
        assert_eq!(count_of_v, builtin_count_of_v);
    }

    #[test]
    fn max_of_empty_list_returns_none() {
        let v: Vec<u64> = Vec::new();
        assert_eq!(find_max(&v), None)
    }

    #[test]
    fn max_of_list_returns_expected_value() {
        let v: Vec<u64> = vec![0, 25, 4, 2, 10, 1, 20];
        assert_eq!(find_max(&v), Some(&25))
    }

    #[test]
    fn max_of_list_with_negative_values_returns_expected_value() {
        let v: Vec<i64> = vec![0, 25, -4, -2, 10, 1, 20];
        assert_eq!(find_max(&v), Some(&25))
    }

    #[test]
    fn max_of_list_with_single_value_returns_expected_value() {
        let v: Vec<i64> = vec![3];
        assert_eq!(find_max(&v), Some(&3))
    }

    #[test]
    fn max_of_empty_list_returns_value_equal_to_builtin_max() {
        let v: Vec<u64> = Vec::new();
        let builtin_max = v.iter().max();
        assert_eq!(builtin_max, find_max(&v))
    }

    #[test]
    fn max_of_list_returns_value_equal_to_builtin_max() {
        let v: Vec<u64> = vec![0, 25, 4, 2, 10, 1, 20];
        let builtin_max = v.iter().max();
        assert_eq!(builtin_max, find_max(&v))
    }
}
