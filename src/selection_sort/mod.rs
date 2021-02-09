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
//!
