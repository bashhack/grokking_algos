//! Selection Sort
//!
//! `selection_sort` is a module introducing memory management, the
//! structural architecture of arrays + linked lists, and demonstrating
//! a basic selection sort algorithm.

//! # Context
//!
//! Developing a mental model of memory management begins with grounding
//! our understanding of computer memory as a collection of bytes
//! associated with integer addresses.
//!
//! Programs can both retrieve and store content to the byte at a given address.
//!
//! Sets of bytes are composed of eight (8) bits, and the allocation of size to
//! common data types can be (like `int`, `float`, `long`, `char`, etc.) expressed
//! in terms of a count of either bits or bytes. As an example, we might sometimes
//! say an `int` is 32 bits, and so can also express an `int` as occupying four (4) bytes.
//! When programs request blocks of memory by address, they often do so using
//! the first byte in the block. So even though an `int` may be located
//! at bytes 10-13, its address would simply be 10.
