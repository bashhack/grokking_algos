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
//! 4.1 Write out the code for the `sum` function
//!
//! 4.2 Write a recursive function to count the number of items in a list
//!
//! 4.3 Find the maximum number in a list
//!
//! 4.4 Remembering binary search - it's a divide and conquer algorithm, too. Can
//!     you come up with the base case and recursive case for binary search?

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
    fn sum_of_non_empty_list_returns_matching_value_to_builtin_sum() {
        let v: Vec<u64> = (0..5).collect();
        let sum_of_v = sum(&v);
        let builtin_sum_of_v = v.iter().sum();
        assert_eq!(sum_of_v, builtin_sum_of_v);
    }
}
