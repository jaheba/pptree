#![feature(associated_type_defaults)]

//! Immutable __cactus stack__ implementation.
//!
//! Other terms for cactus stack include __parent pointer tree__,
//! __spaghetti stack__, and __saguaro stack__. See
//! [wikipedia](https://en.wikipedia.org/wiki/Parent_pointer_tree) for more
//! information.
//!
//! This crate contains two Stack implementations:
//! [`RStack`](struct.RStack.html) and [`VStack`](struct.VStack.html). In short:
//! `RStack` uses a recursive (pointer) architecture, whilst `VStackc` uses a
//! vector to store the stack's data.
//!
//! # Examples
//!
//! ```ignore
//! // trait `Stack` needs to be in scope
//! use kaktus::{Stack, RStack};
//!
//! // tree structure:
//! // 0 -- 1 -- 2
//! //  \
//! //   3 -- 4 -- 5
//!
//! let root = RStack::root(0);
//! let one  = root.push(1);
//! let two  = one.push(2);
//! let five = root.push(3).push(4).push(5);
//!
//! assert_eq!(*five.pop().unwrap(), 4);
//! ```

mod rec;
mod vec;

pub use rec::RStack;
pub use vec::VStack;


pub trait Stack<T> where Self: Sized {
    fn root(val: T) -> Self;
    fn push(&self, val: T) -> Self;
    fn pop(&self) -> Option<Self>;
}
