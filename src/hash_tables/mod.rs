//! Hash Tables
//!
//! `hash_tables` is a module that introduces hash tables - a useful and basic data structure.
//! Central to the exploration of the structure
//!
//! # Applications
//!
//! * Associative arrays (arrays whose indices are arbitrary strings or other objects - a `dictionary` in Python, for example)
//! * Database indexing (also used in disk-based data structures)
//! * Caches (data tables used to speed up data access)
//! * Object representation (Python, JavaScript and Ruby use hash tables to implement objects)
//!
//! # Hash Functions
//!
//! At the core of the hash table is the hash function. At its simplest, a hash function is any function that maps
//! 'data of arbitrary size to fixed-size values' - in other words, quite often a mapping of strings to numbers.
//! These mappings are both consistent (so as to retain like values for like data, ex. 'Van Gogh' is always mapped to '4') and provide
//! some guarantee of uniqueness (so as to map different values to different data, 'Van Gogh' mapping to '4' is fundamentally different than 'Renoir' to '5').
//!
//! For most of us, we are interacting with hash functions any time we operate a computer. These functions, while
//! sharing the aforementioned properties described above, share another trait: they are not functions that almost any
//! of us will have to (nor should) attempt to implement ourselves. The development of a good hash function (algorithms themselves!)
//! is the work of cryptographers - work that is vetted by a community of engineers and transparent in its logic
//! to any who care to ask, because our mutual benefit in using proven and secure hashes greatly outweighs the risks to us all
//! should one implement a bad hash function (i.e., one with high collison rate, etc).
//!
//! For a great overview of hash functions, see: [NIST Special Publication 800-107, Revision 1 - Recommendation for Applications Using Approved Hash Algorithms](https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-107r1.pdf)
//!
//! # Construction
//!
//! Let's start with a simple array structure:
//!
//! [ ][ ][ ][ ][ ]
//!  0  1  2  3  4
//!
//! Our hash table values will occupy indicies within the array after having first passed
//! through a hashing process. Let's walk through the steps:
//!
//! We'll imagine that we storing an arbitrary collection of named entities and some associated
//! set of scalar values for each (ex. 'foo' => 1, 'bar' => 2). The datatypes are arbitrary and
//! we could also store strings, array, other maps, etc.
//!
//! Initially, we want to store `foo`, so we provide `foo` to a hashing function whose return value
//! will provide us with the index we should use to store our value of `1`. Our hash function
//! takes in `foo` and returns an index of `3` - in turn, we store `1` at index `3`.
//!
//! [ ][ ][ ][1][ ]
//!  0  1  2  3  4
//!
//!  Next, we want to store `bar` - it too passes through our hashing function at which point we
//!  receive a return value of `0`. So, again, we store our value `2` at the index provided to us.
//!
//! [2][ ][ ][1][ ]
//!  0  1  2  3  4
//!
//!  This process repeats until all values have been stored:
//!
//! [2][4][3][1][5]
//!  0  1  2  3  4
//!
//!  But - how do we get items out of the hash array? We don't have to search for it iteratively.
//!  Instead, we can simply ask 'please fetch me value `foo`'.
//!
//!             ____________________
//!
//!             |                  |
//!  `foo` ===> |   hash function  | ===> 3 (index)
//!             |                  |
//!             ____________________
//!
//! That's right - we simply pass the value requested back to the hash function! The hash function
//! will return the same value it provided when we leveraged it prior to storage. This, again, is
//! a byproduct of good hash functions, we can call this as many times as necessary and we can be
//! sure that our return value for a given input will be identical.
//!
//! Using the return value, we can perform the lookup in `O(1)` time - very efficient!
//!
//! # Hash Tables in Rust
//!
//! In Rust, hash tables are called hash maps and are provided by the `std::collections` module.
//!
//! We can instantiate from an emtpy map:
//!
//! ```rust
//! use std::collections::HashMap;
//!
//! let mut things = HashMap::new();
//! things.insert("foo", 1);
//! things.insert("bar", 2);
//! things.insert("baz", 3);
//! ```
//!
//! Or even from another data structure:
//!
//! ```rust
//! use std::collections::HashMap;
//!
//! let things: HashMap<&str, i32> = [("foo", 1), ("bar", 2), ("baz", 3)]
//!     .iter().cloned().collect();
//! ```
//!
//! We can fetch values and return them:
//!
//! ```rust
//! use std::collections::HashMap;
//!
//! let things: HashMap<&str, i32> = [("foo", 1), ("bar", 2), ("baz", 3)]
//!     .iter().cloned().collect();
//!
//! if !things.contains_key("banana") {
//!    println!("Found {} things, but no 'banana'", things.len());
//! }
//!
//! things.remove("baz");
//!
//! let to_find = ["foo", "bar"];
//! for &thing in &to_find {
//!     match things.get(thing) {
//!         Some(found) => println!("{}: {}", thing, found),
//!         None => println!("{} is not found.", thing)
//!     }
//! }
//!
//! ```
//!
//! # Exercises
//!
//! Which of these hash function are consistent?
//!
//! 5.1
//!
//! f(x) = 1
//!
//! A - 5.1
//!
//! True
//!
//! ---
//!
//! 5.2
//!
//! f(x) = rand()
//!
//! A - 5.2
//!
//! False
//!
//! ---
//!
//! 5.3
//!
//! f(x) = next_empty_slot()
//!
//! A - 5.3
//!
//! False
//!
//! ---
//!
//! 5.4
//!
//! f(x) = len(x)
//!
//! A - 5.4
//!
//! True
//!
//! # Use Cases In Detail
//!
//! ## Lookups
//!
//! Let's imagine that we want to model a phone book - a simple mapping of name to number.
//!
//! We need to be able to add a name and associated number, as well as be able to enter a name
//! and retrieve their associated number.
//!
//! ```rust
//! use std::collections::HashMap;
//!
//! let mut phone_book = HashMap::new();
//! phone_book.insert("John", 4283058824);
//! phone_book.insert("Ringo", 2123134152);
//! phone_book.insert("George", 3039956721);
//! ```
//!
//! We can also think of DNS resolution in similar terms:
//!
//! ```rust
//! use std::collections::HashMap;
//!
//! let mut dns_store = HashMap::new();
//! dns_store.insert("google.com", 33.125.458.425);
//! dns_store.insert("facebook.com", 852.285.284.4);
//! dns_store.insert("microsoft.com", 12.123.12.123);
//! ```
//!
//! ## Uniqueness
//!
//! There are many times that we face the need to restrict and/or optimize actions against or access to data.
//!
//! We want to build on our phonebook case, ensuring that only a single person with a name is inserted
//! into the hash table. This process might look generally like this:
//!
//!                                     [ Request Made To Insert Data ]
//!                                                    |
//!                            [ Check If This Data Is Already In The Hash Table ]
//!                                  /                                \
//!                    [ YES: Deny Insert ]                    [ NO: Allow Insert ]
//!                                                                     \
//!                                                               [ Add To Hash Table ]
//!
//! If we had been storing these pairs in a simple list, we would face a point where
//! this request to insert data would be unbearably slow, as we'd be performing
//! a simple linear search.
//!
//! See: [linear_search](../intro_to_algos/fn.linear_search.html)
//!
//! ## Collisons
//!
//! When we spoke about what would constitute a bad hash function, one of the defining characteristics
//! was that it would have high collision rates.
//!
//! How do we avoid this possibility?
//!
//! There are two main factors: a robust hash function and a low load factor.
//!
//! Load factor is new to our discussion, but it's simple to grok as it represents:
//!
//!                      number of items in hash table
//!                      -----------------------------
//!                         total number of slots
//!
//! We have already spoken about the construction of hash tables, namely their reliance on arrays for storage.
//! So, looking at the above formula for load factor it's really nothing more than "the count of elements in the array
//! over the length of the array". If we need to store 100 items in our hash table, and we have 100 slots - our load factor
//! is 1. If we have only 50 slots for 100 items, our load factor is doubled - it's 2! We don't have enough slots for the
//! items we intend to store. If we have a load factor greater than 1, we will need to resize our hash table to avoid
//! collisions. However - resizing is a cost that we should be mindful of, it's expensive and we should aim to minimize
//! the need to do so.
//!
//! The second key to low collision rates we said was a good hash function - in essence, the quality of this function
//! depends in part on its ability to evenly distribute values within the array storage. SHA (Secure Hash Algorithm)
//! functions, in particular, are often safe bets. Just be sure to use the latest recommended SHA function(s) to ensure
//! safe cryptographic hashing!
//!
//! # Exercises
//!
//! Suppose you have these four hash functions that work with strings:
//!
//! A. Return '1' for all input
//!
//! B. Use the length of the string as the index
//!
//! C: Use the first character of the string as the index. So, all strings starting with 'a' are hashed
//!    together, and so on.
//!
//! D: Map every letter to a prime number: a = 2, b = 3, c = 5, d = 7, e = 11, and so on. For a string,
//!    the hash function is the sum of all the characters modulo the size of the hash. For example,
//!    if your hash size is 10, and the string is "bag", the index is 3 + 2 + 17 % 10 = 22 % 10 = 2.
//!
//! For each of these examples, which hash functions would provide a good distribution? Assume a hash table
//! size of 10 slots.
//!
//! 5.5
//!
//! A phonebook where the keys are names and values are phone numbers. The names are as follows:
//! Esther, Ben, Bob and Dan.
//!
//! A - 5.5
//!
//! C + D
//!
//! ---
//!
//! 5.6
//!
//! A mapping from battery size to power. The sizes are A, AA, AAA, and AAAA.
//!
//! A - 5.6
//!
//! B + D
//!
//! ---
//!
//! 5.7
//!
//! A mapping from book titles to authors. The titles are: Maus, Fun Home, and Watchmen.
//!
//! A - 5.7
//!
//! B + C + D
