//! Intro to Algorithms
//!
//! `intro_to_algos` is a module that serves to introduce the foundational
//! concepts and modes of analysis that will ground future algorithmic
//! study.

//! # Definitions
//!
//! `algorithm` - a set of instructions to accomplish a given task
//!
//! # Context
//!
//! These instructions may take the form of the familiar - Dijkstra's algorithm,
//! Binary Search, N Queen Problem, etc - or it may be almost any block of
//! code.
//!
//! In any case, an algorithm is a set of finite instructions acting upon
//! a set of inputs to produce output (that may sometimes include returning `None`).
//!
//! How an algorithm behaves for a given set of inputs is captured
//! in the work of algorithmic analysis. Specifically, we'll often refer
//! to 'how fast' an algorithm is by studying the vectors of time + space.
//! That is, how long will this algorithm execute and what spatial
//! resources (i.e., memory, disk spaces, etc.) are required for its execution.
//!
//! # Big O Notation
//!
//! When we ask about the speed of an algorithm, we commonly use a form
//! of special notation known as `Big O Notation`.
//!
//! In short, it's a convenient way for us to speak in a shared
//! language of sorts - so that no matter the algorithm, no matter its
//! author, and no matter the language or domain in which it has been
//! implemented - we can understand the expected performance.
//!
//! But is that the expected performance in the ideal case? No!
//!
//! It is important to remember that Big O notation is intended to capture
//! the worst-case run time. While this might seem like a negative - we
//! can in fact find comfort in knowing that upper-bound, as we can't
//! encounter something slower.
//!
//! Enough of this introduction - how can we start to comprehend Big O?
//!
//! ## Examples
//!
//! A common classification of algorithms for dipping one's toes into
//! Big O notation is the time-honored search algorithms. These include
//! linear search, binary search, jump search, and many others.
//!
//! The variance between these algorithms can be significant! Let's take a look:
//!
//! Imagine for a moment we have a simple vector of `N=100` elements allocated on the 'heap':
//!
//! ```
//! let v = vec![0, 1, 2, ... 100];
//! ```
//!
//! If we use linear search, for example, and we have a minimum lookup of 1ms per element,
//! we know we can expect a 100ms time to run. Why is that? Because in linear search
//! we will visit at most every element in sequence.
//!
//! Now, let's imagine we use binary search instead. Here, we now have something far more efficient
//! - on the order of maybe 7ms. In this example, this is because we have only visited
//! at most seven (7) elements. We can think about this in logarithmic terms as log<sub>2</sub>100.
//! As a refresher - 7 here serves an approximate value for '2 raised to what power is equal to 100'.
//!
//! Quite the difference, right? How does this scale as our `N` grows? What happens when we have
//! `N=1000000000`?
//!
//! For such a large input, we can guess that binary search would yield around
//! 30ms (log<sub>2</sub>1000000000 is approximately 30). Again, in contrast, a linear
//! search would have yielded a staggering 11+ days (1000000000 x .001 seconds)!
//!
//! This drastic difference illustrates an important truth: the run times of these (and many)
//! algorithms grow at vastly different rates depending on their input.
//!
//! Had we simply assumed that our results at `N=100` (linear: 100ms / binary: 7ms = ~14x faster in
//! the binary search case) held true for `N=1000000000` (linear: Xms / binary: 30ms) we could
//! have erroneously thought 30ms x ~14 = 420ms was a reasonable run time in the linear case and
//! been met with an unpleasant surprise over a week later when our algorithm actually finished.
//!
//! This example is a small indication of the importance of understanding how our
//! algorithms run, but knowing that as the size of our inputs scales. Big O notation
//! can help us express the exact phenomenon we've investigated! After all,
//! it serves to help us conceptualize the operations required for an algorithm to execute -
//! this in turn can help us visualize the time + space requirements.
//!
//! Returning to the linear search example, for some vector of size `N` we know
//! we will visit - in the worst case - every element. We say then that the run time in Big O is
//! O(n) where `n` is the number of operations.
//!
//! We saw that binary search was a logarithmic reduction of operations, so we might express its
//! complexity as O(log n).
//!
//! ## Common Big O run times
//!
//! Fastest to slowest...
//!
//! * O(log n) - `log time` - binary search
//!
//! * O(n) - `linear time` - linear search
//!
//! * O(n * log n) - typical fast sort algorithm - quicksort
//!
//! * O(n<sup>2</sup>) - typical slow sort algorithm - selection sort
//!
//! * O(n!) - very slow `factorial time` algorithm - traveling salesperson
//!
//! # Exercises
//!
//! 1.3 You have a name, and you want to find the person's phone number in the phone book.
//!     A: O(log n)
//! 1.4 You have a phone number, and you want to find the person's name in the phone book.
//!     A: O(n)
//! 1.5 You want to read the numbers of every person in the phone book.
//!     A: O(n)
//! 1.6 You want to read the numbers of just the As.
//!     A: O(n/26) => should reduce to just O(n) as we always simplify
//!
//! # Binary Search in Detail
//!
//! As a simple example, let's first look at how we might describe the algorithm in pseudo-code:
//!
//! // Given a sorted list and an item,
//! // if the item is in the list - return the item.
//! //
//! // Initially, we define a low value of 0 and a high value equal to the list's length minus 1.
//! //
//! // Then, `while` we haven't found the element (`low <= high`), we
//! // check the middle element "our guess" (`(low + high) // 2`) against the following:
//! //
//! // - If the guess is equal to the item, return the item.
//! // - If the guess is not equal to the item,
//! //       if the guess is greater than the item, reset the high so that `high = mid - 1`
//! //       if the guess is less than the item, reset the low so that `low = mid + 1`

// An iterative binary search
pub fn iterative_binary_search() {
}