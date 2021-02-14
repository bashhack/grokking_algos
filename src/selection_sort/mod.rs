//! Selection Sort
//!
//! `selection_sort` is a module introducing memory management, the
//! structural architecture of arrays + linked lists, and demonstrating
//! a basic selection sort algorithm.

//! # Memory
//!
//! Developing a mental model of memory management begins with grounding
//! our understanding of computer memory as a collection of bytes
//! associated with integer addresses.
//!
//! Programs can both retrieve and store content to the byte at a given address.
//! An address is also commonly known as a pointer - so named because a pointer
//! references or points to a specific byte position in memory.
//!
//! The content stored by a program or the operation system to random-access
//! memory (RAM) can include:
//!
//!   - program bytecode queued for execution
//!   - data values and data structures used by the program
//!   - runtime systems/runtime environments
//!
//! It's worth asking - _what is a byte anyway?_ Put plainly, a single byte is
//! comprised of eight (8) bits. Our common data types can be (`int`, `float`,
//! `long`, `char`, etc.) expressed in terms of a count of either bits or bytes.
//!
//! As an example, we might sometimes say an `int` is 32 bits, and so can also
//! express an `int` as occupying four (4) bytes. When programs request blocks
//! of memory by address, they often do so using the first byte in the block.
//! So, even though an `int` may be located at bytes 10-13, its address would be 10.
//!
//! ## Areas of Memory
//!
//! We've briefly covered the core building blocks of memory - the bit and the byte -
//! but as we continue to expand our concept of the memory model the abstractions
//! expand, as well.
//!
//! Most notably, data being stored to memory apart from bytecode, tends to
//! be stored in one of two commonly identified partitions: the stack and the heap.
//!
//! ### Stack
//!
//! The stack is characterized by:
//!   
//!   - fast access lookup
//!   - data is removed via "last-in first-out (LIFO)" strategy
//!   - stored data is static and finite in size
//!   - stored data must be known at compile-time
//!   - leverages stack frames
//!   - stacks can be allocated per thread for multi-threaded applications
//!   - data stored includes local variables, pointers, and function frames
//!   - some values can only be stored on heap due to the size constraints
//!   - origin of stack overflow errors
//!
//! ### Heap
//!
//! The heap is characterized by:
//!
//!   - slow access lookup
//!   - uses pointers as mechanism for lookup
//!   - data with dynamic sizing can be stored
//!   - shared access to data across threads
//!   - because it is dynamic, often more complex to manage and source of pointer bugs
//!   - data stored includes global variables, reference types like strings, maps, and objects
//!   - origin of out of memory errors
//!
//! ## Memory Management
//!
//! Programming languages also introduce their own layer of abstraction to the memory model by
//! way of their memory management strategies.
//!
//! Those familiar with the hazards and wizardry of C/C++ memory management will already be
//! aware of the first of these: manual memory management.
//!
//! Modern languages commonly use garbage collection (either mark + sweep (Ruby, JavaScript, JVM)
//! or reference counting (PHP, Perl, Python)). In this model, a process periodical
//! identifies objects in memory that can be removed.
//!
//! A variant of reference counting garbage collection - automatic reference counting - can be
//! found in Objective-C, Swift and the Clang compiler.
//!
//! Finally, we have "resource acquisition is initialization (RAII)" - in this strategy, used by
//! C++, Ada, Rust, memory allocation is driven by an object's lifetime (the span of time between
//! its construction and destruction).
//!
//! ### Memory Management in Rust
//!
//! This is a deep subject, and one better covered by the following primary resources:
//!
//!   - [What Is Ownership?](https://github.com/rust-lang/book/blob/fc9ca867ea6372fc696e1fb4f1b996dcb51f4954/src/ch04-01-what-is-ownership.md)
//!   - [References and Borrowing](https://github.com/rust-lang/book/blob/fc9ca867ea6372fc696e1fb4f1b996dcb51f4954/src/ch04-02-references-and-borrowing.md)
//!   - [Fearless Security: Memory Safety](https://hacks.mozilla.org/2019/01/fearless-security-memory-safety/)

//! # Data Structures: Arrays + Linked Lists
//!
//! Two of the most common data structures used for storing items in memory are arrays and lists.
//!
//! Sharing many similarities, they differ in that arrays store their data in contiguous blocks,
//! whereas linked lists are a collection of address blocks where each item stores a reference to
//! the address of the previous item in the list.
//!
//! The performance characteristics (and tradeoffs) between them are typically described in terms
//! of the relative ease with which each handles appending new items or removing existing items.
//! Linked lists excel when handling inserts over reads while arrays excel at random reads
//! but fall short for inserts due to the need to reallocate memory. Deletion behavior
//! follows insertion trends - with lists outperforming arrays in the matter, as deletion
//! prompts remaining elements in an array to be moved.
//!
//! In short, we can summarize the performance characteristics between arrays + lists
//! in general terms as follows:
//!
//! | Operation   | Arrays      | Lists      |
//! | ----------- | ----------- | ---------- |
//! | Read        | O(1)        | O(n)       |
//! | Insert      | O(n)        | O(1)       |
//! | Deletion    | O(n)        | O(1)       |
//!
//! Typically, arrays see more usage because of their fast constant time random access reads.
//! Linked lists can only provide sequential access, resulting in a request for lookup of the
//! element located at the `n` position of linked list requiring a read of all prior elements
//! before `n`.
//!
//! ## Exercises
//!
//! 2.1 Suppose you're building an app to keep track of your finances. Every day, you write
//!     down everything you spent money on. At the end of the month, you review your expenses
//!     and sum up how much you spent. So, you have lots of inserts and a few reads. Should
//!     you use an array or a list?
//!
//!     A. If we have more inserts than reads, we would be wise to leverage lists. We are
//!        in luck for the read requirement as well - our problem statement involves a summing
//!        operation which we could deem sequential reads (as opposed to "random access" reads).
//!        Sequential reads for linked lists can be quite performant, so a linked lists
//!        would be an appropriate structure.
//!
//! 2.2 Suppose you're building an app for restaurants to take customer orders. Your app needs
//!     to store a list of orders. Servers keep adding orders to this list, and chefs take orders
//!     off the list and make them. It's an order queue: servers add orders to the back of the
//!     queue, and the chef takes the first order off the queue and cooks it. Would you use an
//!     array or a linked list to implement this queue?
//!
//!     A. Again, a linked list here is ideal - there's no random access reads necessary, as we're
//!        dealing with a simple "first in first-out (FIFO)" lookup strategy. These insertion
//!        and deletion actions on the queue favor the constant time characteristics provided
//!        by a linked list's innate tendency to track its first and last elements.
//!
//! 2.3 Suppose Facebook keeps a list of usernames. When someone tries to log in to Facebook,
//!     a search is done for their username. If their name is in the list of usernames,
//!     they can log in. People log in to Facebook pretty often, so there are a lot of searches
//!     through this list of usernames. Suppose Facebook uses binary search to search the list.
//!     Binary search needs random access - you need to be able to get to the middle of the list
//!     of usernames instantly. Knowing this, would you implement the list as an array or a linked
//!     list?
//!
//!     A. _you need to be able to get to the middle of the list of usernames instantly_ is our
//!        cue that we need an array - preferably sorted, as our binary search algorithm
//!        requires it!
//!
//! 2.4 People sign up for Facebook pretty often, too. Suppose you decided to use an array to
//!     store the list of users. What are the downsides of an array for inserts? In particular,
//!     suppose you're using binary search to search for logins. What happens when you add new
//!     users to an array?
//!
//!     A. The biggest downside is the potential for memory reallocation. This could incur
//!        a performance overhead in and of itself, but should we have exhausted available space
//!        in memory, we might even risk out of memory errors. This is to say nothing of
//!        the need to insure the inserted element is then sorted into its proper position.
//!
//! 2.5 In reality, Facebook uses neither an array nor a linked list to store user information.
//!     Let's consider a hybrid data structure: an array of linked lists. You have an array with
//!     26 slots. Each slot points to a linked list. For example, the first slot in the array
//!     points to a linked list containing all the usernames starting with `a`. The second slot
//!     points to a linked list containing all the usernames starting with `b`, and so on.
//!
//!     Suppose Adit B signs up for Facebook, and you want to add them to the list. You go to
//!     slot 1 in the array, go to the linked list for slot 1, and add Adit B at the end. Now,
//!     suppose you want to search for Zakhir H. You go to slot 26, which points to a linked list
//!     of all the Z names. Then you search through that list to find Zakhir H.
//!
//!     Compare this hybrid data structure to arrays and linked lists. Is it slower or faster
//!     than each for searching and inserting? You don't have to give Big O run times,
//!     just whether the new data structure would be faster or slower.
//!
//!     A. Our hybrid data structure strikes a balance between arrays + linked lists. Arrays
//!        provide excellent reads over linked lists - so our hybrid structure accelerates
//!        the initial lookup penalty we'd otherwise expect from a linked list or a list
//!        of linked lists. Our hybrid data structure also gains the efficiency of
//!        insertion/deletion as well - a penalty we would have incurred with an array of arrays.

//! # Selection Sort
//!
//! We have previously looked at binary search, but we now turn our attention to our first sorting
//! algorithm: _selection sort_. This novel algorithm is an entry point to a large number of
//! sorting algorithms, including quicksort and others.
//!
//! Do note that if you are after an efficient sorting algorithm, look elsewhere first!
//!
//! # Selection Sort in Detail
//!
//! Let's imagine we are tasked with sorting a list of unordered integers - for this simple
//! example, let's first look at how we might describe the algorithm in pseudo-code.
//!
//! ```text
//! Given a list of unordered elements, find the smallest element, placing it at the head
//! of the list. Proceed to find the next min value ahead of just recently placed element.
//! Repeat this process until the list has been sorted smallest to largest.
//! ```
//!
//! How would we describe the performance characteristics of this algorithm?
//! We can immediately spot that asking for the min of a list would require us to visit
//! each element in the list once, meaning we can deduce this operation takes `O(n)` time.
//! For a list of size `n`, we repeat the search for min (i.e., we could also use max or any
//! another predicate comparator depending on the nature of the sort logic) `n` times.
//! In short, we perform `O(n)` `n` times - in other words `O(n * n)` or O(n<sup>2</sup>).

/// A selection sort
///
/// Given an unordered list, sorts it from smallest to largest in-place - returns an ordered vector.
///
/// Should be expected to have performance characteristics of `O(n * n)`.
///
/// # Arguments
///
/// * `list` - An unsorted vector of elements
///
/// # Examples
///
/// ```rust
/// let mut list = [5, 2, 1, 6, -1];
/// selection_sort(&mut list)
/// ```
pub fn selection_sort<T: Ord>(list: &mut [T]) -> &mut [T] {
    // Initialize an index counter and store list length
    let (mut i, len) = (0, list.len());

    // As long as we have a value to iterate, do so
    while i < len {
        // Store reference to next element in list as j,
        // and existing index counter from outer scope as
        // the current min value
        let (mut j, mut current_min) = (i + 1, i);

        // Iterate (or scan) ahead across the length of the list
        while j < len {
            // Asking whether the element ahead of the previous element
            // is less than our current smallest (or min) value
            if list[j] < list[current_min] {
                // If it is less, reassign our current min value
                current_min = j;
            }
            // Increase j - moving iteration forward
            j = j + 1;
        }

        // Swap current index with current min value
        list.swap(i, current_min);
        // Increase i - moving iteration forward
        i = i + 1;
    }
    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_sort_returns_none_if_provided_empty_list() {
        let mut v: [&str; 0] = [];
        assert_eq!(selection_sort(&mut v), [] as [&str; 0])
    }

    #[test]
    fn selection_sort_correctly_sort_unordered_list() {
        let mut v = [2, 1, 3, 0, 4];
        assert_eq!(selection_sort(&mut v), [0, 1, 2, 3, 4]);
    }

    #[test]
    fn selection_sort_returns_existing_ordered_list() {
        let mut v = [0, 1, 2, 3, 4];
        assert_eq!(selection_sort(&mut v), [0, 1, 2, 3, 4])
    }

    #[test]
    fn selection_sort_handles_repeating_values() {
        let mut v = [5, 3, 0, 2, 1, 3, 4, 5];
        assert_eq!(selection_sort(&mut v), [0, 1, 2, 3, 3, 4, 5, 5])
    }

    #[test]
    fn selection_sort_works_over_list_of_strings() {
        let mut v = ["cat", "apple", "zebra", "banana"];
        assert_eq!(selection_sort(&mut v), ["apple", "banana", "cat", "zebra"])
    }
}
