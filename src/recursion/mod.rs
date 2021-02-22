//! Recursion
//!
//! `recursion` is a module that introduces the high-level mechanics of the powerful and
//! elegant problem solving method we call recursion - in essence, a method of arriving to a result
//! by way of reducing a problem into smaller problems of the same general form.
//!
//! # Context
//!
//! We sometimes hear about recursion and recoil - imagining it is too hard, too abstract, or
//! too slow. There are reasons to put consider each of these critiques, but there are as many
//! reasons to dispel them.
//!
//! For now, let's talk about recursion as a concept rather than the implementation (as with any
//! technique, application and implementation is everything if using it successfully!). Should
//! the question of proper usage of recursion or its merits be on the table - one only
//! has to ask, and I'll extol the benefits!
//!
//! # Mechanics
//!
//! Before we look at code or even pseudocode - let's identify the parts of recursion:
//!
//! * A base case
//! * A winding step
//! * An unwinding step
//!
//! ## The Base Case
//!
//! The point at which the function must return, and the calling of itself is ceased. This is
//! typically the first part of your recursive procedure.
//!
//! ## The Winding Step
//!
//! The point at which the recursive function is first called, ending with the entry into the
//! base case. The winding step involves no return statements, and so we traverse further into
//! the call stack, again and again - wound up like a ready to uncoil itself.
//!
//! ## The Unwinding Step
//!
//! Usually this occurs just after the winding - it begins when the base case is hit, and
//! it ends when the very first call to the procedure finally returns.
//!
//! We'll illustrate these steps further below in psuedocode.
//!
//! # The Why of Recursion
//!
//! So, why would one even want to use recursion over the iterative solution to a problem?
//!
//! For some problems, the recursive solution is the natural one precisely because the problem is
//! naturally recursive. Evaluating the factorial of a value is a classic computer science
//! favorite for demonstration - though there are others, of course. Finding palindromes, the
//! greatest common divisor or evaluating the Fibonacci sequence, Ackermann's function, or
//! Pascal's triangle are still but a few.
//!
//! Likewise, many data structures (and structures in the natural world) have recursive
//! properties - including, trees, graphs, and even the as-ever-useful list. Working
//! against them one may find they yield solutions most elegantly expressed recursively.
//!
//! Familiar to almost anyone surely reading this - the filesystems on our computers and
//! portable devices is also an example of a recursive data structure!
//!
//! Recursion can also be the correct approach to solve a problem in order to take advantage of
//! immutability. In fact, this close relationship with immutable data and recursion is central
//! to the paradigm of functional programming, with both of these fundamental qualities lending
//! to programs that can be far easier to reason about compared to their imperative counterparts.
//! Evidence suggests that recursion is often less error-prone and often of smaller code-size -
//! these benefits extend generally to the languages that rely on it, as well, including
//! GHC Haskell, Clojure, Scala, Scheme, F#, Agda, etc.
//!
//! The downside to the usage of recursion is most often in its space requirements. As the recursive
//! calls are stacked one atop the other, memory is used; and, as memory is consumed, once there
//! is no further room for allocation we might have exhausted resources before arriving to
//! our solution.
//!
//! # Recursion In Detail
//!
//! Let's look at an example now
//! The pseudocode might look like:
//!
//! ```text
//! my_recursive_procedure(my_arg):
//!     // This is the winding phase
//!     do_something_here_with(my_arg)
//!
//!     // This is our base case
//!     if (length(my_arg) == 0):
//!         return a_value;
//!
//!     // This is the recursive call + the start of the unwinding step
//!     my_recursive_procedure(my_arg_but_altered_to_ensure_we_will_converge_toward_base_case)
//! ```
//!
//! ## Exercises
//!
//! 3.1
//!
//! Suppose we have call stack that looks like:
//!
//! ```text
//! | ----------- | ----------- |
//! |          Greet 2          |
//! | ----------- | ----------- |
//! | Name:       |   Winston   |
//! | ----------- | ----------- |
//! |           Greet           |
//! | ----------- | ----------- |
//! | Name:       |   Winston   |
//! | ----------- | ----------- |
//! ```
//!
//! What can say about the current state of the call stack?
//!
//! A - 3.1
//!
//! We can determine that there are two functions on the stack: `Greet` and `Greet 2`.
//! Each takes at least a single parameter (`name`) having a value of `Winston`.
//! Both functions being on the stack would indicate that we in the middle of the
//! stack evaluation. Once `Greet 2` is called, we can expect that `Greet` will resume
//! its execution.
//!
//! ---
//!
//! 3.2
//!
//! Suppose you accidentally write a recursive function that runs forever. As you
//! saw, your computer allocates memory on the stack for each function call.
//! What happens to the stack when your recursive function runs forever?
//!
//! A - 3.2
//!
//! Because the stack is a limited resource - we are going to be headed toward a stack overflow!
//! This is absolutely not a good error to encounter - in fact, we often classify it as a
//! fatal error. Once we detect a stack overflow error, we cannot guarantee the integrity
//! of the program state and more often than not, our program has likely crashed completely.

/// A simple recursive countdown
///
/// Iterates down from a value until zero is reached - returns zero (`0`)
///
/// Should be expected to have performance characteristics of `O(n)`.
///
/// # Arguments
///
/// None
///
/// # Examples
///
/// ```rust
/// simple_recursive_countdown(10)
/// ```
pub fn simple_recursive_countdown(i: i32) -> i32 {
    println!("{}", i);
    // Our base case
    if i <= 0 {
        return i;
    }
    // Our recursive case
    simple_recursive_countdown(i - 1)
}

/// A greeter
///
/// Given a name input - returns a series of greeting phrases
///
/// Useful in examing the mechanics of the call stack:
///
///
/// ```text
/// | ----------- | ----------- |
/// |                           |
/// | ----------- | ----------- |
///
///
///
/// | ----------- | ----------- |
/// |           Greet           |
/// | ----------- | ----------- |
/// | Name:       |   Winston   |
/// | ----------- | ----------- |
///
///
///
/// | ----------- | ----------- |
/// |          Greet 2          |
/// | ----------- | ----------- |
/// | Name:       |   Winston   |
/// | ----------- | ----------- |
/// |           Greet           |
/// | ----------- | ----------- |
/// | Name:       |   Winston   |
/// | ----------- | ----------- |
///
///
///
/// | ----------- | ----------- |
/// |            Bye            |
/// | ----------- | ----------- |
/// |           Greet           |
/// | ----------- | ----------- |
/// | Name:       |   Winston   |
/// ```
///
///
/// # Arguments
///
/// * `name` - a name used when greeting
///
/// # Examples
///
/// ```rust
/// greeter("Winston")
/// ```
pub fn greeter(name: &str) {
    println!("hello {}!", name);
    second_greeter(name);
    println!("getting ready to say bye...");
    bye()
}

#[doc(hidden)]
fn second_greeter(name: &str) {
    println!("how are you, {}?", name)
}

#[doc(hidden)]
fn bye() {
    println!("ok bye!")
}

/// A factorial implementation
///
/// Given an integer, attempts to evaluate its factorial - returns a factorial
///
/// Should be expected to have performance characteristics of fatorial time: `O(n!)`.
///
/// # Arguments
///
/// * `i` - an unsigned integer
///
/// # Examples
///
/// ```rust
/// factorial(10)
/// ```
pub fn factorial(i: u64) -> u64 {
    match i {
        0 => 1,
        _ => factorial(i - 1) * i,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_to_zero_for_positive_input() {
        assert_eq!(simple_recursive_countdown(10), 0)
    }

    #[test]
    fn resolves_to_original_input_for_negative_input() {
        assert_eq!(simple_recursive_countdown(-10), -10)
    }

    #[test]
    fn greets() {
        // To see output during test suite:
        // cargo test greets -- --show-output
        greeter("Churchill")
    }

    #[test]
    fn finds_factorial_for_max_input() {
        assert_eq!(factorial(20), 2432902008176640000)
    }

    #[test]
    fn finds_factorial_for_input() {
        assert_eq!(factorial(5), 120)
    }

    #[test]
    fn finds_factorial_for_one() {
        assert_eq!(factorial(1), 1)
    }

    #[test]
    fn find_factorial_for_zero() {
        assert_eq!(factorial(0), 1);
    }
}
