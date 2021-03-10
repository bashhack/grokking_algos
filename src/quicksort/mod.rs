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
//! ## Continued Exercises
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
//! See: [recursive_binary_search](../intro_to_algos/fn.recursive_binary_search.html)
//!
//! # Intro to Quicksort
//!
//! Like [selection_sort](../selection_sort/fn.selection-sort.html) before it,
//! Quicksort is common and simple sorting algorithm. Its advantage over the former
//! is in its speed - it is a faster algorithm than selection, having an expected
//! runtime performance of `O(n log n)`. In fitting with the theme of this module,
//! it is also a divide and conquer algorithm.
//!
//! # Quicksort in Detail
//!
//! ```text
//! Given an array of elements, while there are elements, iterate over the elements.
//! If there is a value at the moment of iteration, let this the pivot element.
//! Partition the array of elements, into two sub-arrays
//! (elements less than the pivot and elements greater than the pivot, respectively).
//! Then, call Quicksort recursively on each of the two sub-arrays.
//! Finally, combined the sorted sub-arrays on either side of the pivot element and return.
//! ```
//!
//! ## Perils of the Pivot
//!
//! We have described Quicksort as an algorithm that can be expected to have the
//! performance characteristics of `O(n log n)` - but, crucially, this is only in the
//! average case. Selection sort, as we saw previously, had a Big O of `O(n * n)`.
//! As it happens this is also the worst case performance for Quicksort, as well.
//!
//! Interestingly, Quicksort's neighboring sort algorithm: merge sort is actually consistenly
//! `O(n log n)`. The question then is why not just use merge sort over Quicksort
//! all the time?
//!
//! The answer lies in the value of the Big O constant values between them. Merge sort
//! has a larger constant value than quicksort, making it slower in practice than Quicksort.
//! Additionally, quicksort is generally faster than merge sort in practice because
//! it will tend to perform in the average case more often than the worst case.
//!
//! But what consitutes the worst case, anyway? The pivot element in Quicksort bears
//! heavy impact on its performance. Thankfully the worst-case is not as frequently
//! encountered in practice, but let's imagine that we have picked the first element
//! in the list as our pivot element (on a related note, most implementations of the
//! algorithm choose either the first or last). Now, let's also imagine we have a sorted
//! list. Because the Quicksort algorithm eagerly begins its execution without checking
//! if the list is sorted, we will still attempt to sort the sorted list.
//!
//! When this occurs, we will have a longer call stack compared to a middle-point pivot
//! against that same sorted list:
//!
//! ```text
//! [1] [2] [3] [4] [5] [6] [7] [8]
//!
//! // NOTE: We pick the first element as the pivot every time
//! [ ] <1> [2] [3] [4] [5] [6] [7] [8]
//!         [ ] <2> [3] [4] [5] [6] [7] [8]
//!             [ ] <3> [4] [5] [6] [7] [8]
//!                 [ ] <4> [5] [6] [7] [8]
//!                     [ ] <5> [6] [7] [8]
//!                         [ ] <6> [7] [8]
//!                             [ ] [7] [8]
//! ```
//!
//! Compare this with a middle-point pivot against that same sorted list:
//!
//! ```text
//! [1] [2] [3] [4] [5] [6] [7] [8]
//!
//! [1] [2] [3] <4> [5] [6] [7] [8]
//! [1] <2> [3]         <6> [7] [8]
//!                     [ ] <7> [8]
//! ```
//!
//! In the first case, we have the worst case runtime and in the second we have
//! the best-case. In other words, the stack size of the first is `O(n)` and the second is `O(log n)`.
//! For both, the time at each level is `O(n)`. Extrapolating this stack size by level time,
//! we have Big O of `O(n * n)` and `O(n log n)`, respectively.
//!
//! ## Exercises
//!
//! 4.5
//!
//! Printing the value of each element in an array.
//!
//! A - 4.5
//!
//! `O(n)`
//!
//! ---
//!
//! 4.6
//!
//! Doubling the value of each element in an array.
//!
//! A - 4.6
//!
//! Also, `O(n)`.
//!
//! ---
//!
//! 4.7
//!
//! Doubling the value of just the first element in an array.
//!
//! A - 4.7
//!
//! `O(1)`
//!
//! ---
//!
//! 4.8
//!
//! Creating a multiplication table with all the elements in the array. So if your array is [2, 3, 7, 8, 10],
//! you first multiply every element by 2, then multiply every element by 3, then by 7, and so on.
//!
//! A - 4.8
//!
//! `O(n * n)` or O(n<sup>2</sup>)

use std::{cmp::max, iter::once};

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
/// let v: Vec<u64> = (0..5).collect();
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
/// let v: Vec<u64> = (0..5).collect();
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
/// let v: Vec<u64> = (0..5).collect();
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
/// Returns either the max value of the list or None.
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
/// let v: Vec<u64> = (0..5).collect();
/// find_max(&v)
/// ```
pub fn find_max<T: Ord + Copy>(list: &[T]) -> Option<&T> {
    let index = list.len().checked_sub(1).unwrap_or(0);
    _find_max(&list, index)
}

/// A quicksort implementation
///
/// Given a list, sorts the elements therein - returns a sorted list.
///
/// Should be expected to have average performance characteristics of `O(n log n)`.
/// But in the worst case, quicksort may be `O(n * n)` or O(n<sup>2</sup>).
///
/// # Arguments
///
/// * `list` - a list of elements (can be empty list)
///
/// # Examples
///
/// ```rust
/// let v: Vec<u64> = vec![2, 0, 3, 4, 5, 1];
/// quick_sort(v.clone().into_iter())
/// ```
pub fn quick_sort<T, E>(mut list: T) -> Vec<E>
where
    T: Iterator<Item = E>,
    E: PartialOrd,
{
    // Given an array of elements,
    // while there are elements, iterate over the elements.
    match list.next() {
        None => Vec::new(),

        // If there is a value at the moment of iteration,
        // let this the pivot element.
        Some(pivot) => {
            // Partition the array of elements, into two sub-arrays
            // (elements less than the pivot and elements greater than the pivot, respectively).
            let (list_of_smaller, list_of_greater): (Vec<_>, Vec<_>) =
                list.partition(|n| n < &pivot);

            // Then, call quicksort recursively on each of the two sub-arrays.
            let sorted_small = quick_sort(list_of_smaller.into_iter());
            let sorted_greater = quick_sort(list_of_greater.into_iter());

            // Finally, combined the sorted sub-arrays on either side of the pivot element and return.
            sorted_small
                .into_iter()
                .chain(once(pivot))
                .chain(sorted_greater.into_iter())
                .collect()
        }
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

    #[test]
    fn quick_sort_of_empty_list() {
        let v: Vec<u64> = Vec::new();
        assert_eq!(quick_sort(v.clone().into_iter()), v)
    }

    #[test]
    fn quick_sort_of_one_element_list() {
        let v: Vec<u64> = vec![1];
        assert_eq!(quick_sort(v.clone().into_iter()), v)
    }

    #[test]
    fn quick_sort_of_four_element_list() {
        let v: Vec<u64> = vec![2, 1, 4, 0, 3, 5];
        assert_eq!(quick_sort(v.clone().into_iter()), vec![0, 1, 2, 3, 4, 5])
    }

    #[test]
    fn quick_sort_of_four_element_list_with_negative_values() {
        let v: Vec<i64> = vec![2, 1, 4, 0, 3, -5];
        assert_eq!(quick_sort(v.clone().into_iter()), vec![-5, 0, 1, 2, 3, 4])
    }

    #[test]
    fn quick_sort_of_four_element_list_with_str_values() {
        let v: Vec<&str> = vec!["mangoes", "apple", "apples", "bananas"];
        assert_eq!(
            quick_sort(v.clone().into_iter()),
            vec!["apple", "apples", "bananas", "mangoes"]
        )
    }

    #[test]
    fn quick_sort_of_repeated_values() {
        let v: Vec<u64> = vec![5, 2, 4, 1, 0, 1, 3, 4];
        assert_eq!(
            quick_sort(v.clone().into_iter()),
            vec![0, 1, 1, 2, 3, 4, 4, 5]
        )
    }
}
