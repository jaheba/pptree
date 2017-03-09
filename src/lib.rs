#![feature(associated_type_defaults)]

//! Immutable __cactus stack__ implementation.
//!
//! Other terms for cactus stack include __parent pointer tree__,
//! __spaghetti stack__, and __saguaro stack__. See
//! [Wikipedia](https://en.wikipedia.org/wiki/Parent_pointer_tree) for more
//! information.
//!
//! ```ignore
//! // Quickstart
//! extern crate kaktus;
//! // the trait `Stack` needs to be importet for `Stack`/`VStack` to work
//! use kaktus::{Stack, Stack};
//!
//! let root = Stack::root(0);
//! let one = root.push(1);
//! assert_eq!(*one.pop().unwrap(), 0);
//! assert_eq!(*one, 1);
//! ```
//!
//! # Overview
//!
//! The stacks described in this crate differ from traditional stacks in one
//! decisive point, they are *immutable*. This means that a value in itself
//! represents the stack:
//!
//! ```ignore
//! let root = Stack::root(0);
//! let one = root.push(1);
//! let two = root.push(2);
//! assert_eq!(*two, 2);
//! ```
//! Further, popping a value from the stack just returns the parent -- the
//! originial value (and thus the stack it represents) remains valid:
//!
//! ```ignore
//! let one_ = two.pop().unwrap();
//! assert_eq!(*one_, 1);
//! // `two` is still valid
//! assert_eq!(*two, 2);
//! ```
//!
//! For comparison, this shows how stacks are often implemented instead:
//!
//! ```ignore
//! // traditional stack
//! let mut stack = vec![0];
//! stack.push(1);
//! stack.push(2);
//! let two = stack.pop().unwrap();
//! let one = stack.pop().unwrap();
//! ```
//!
//! ## Cactus stacks
//!
//! Due to the immutable property, it is possible to spawn off multiple values
//! from the same parent, making it effecively a tree:
//!
//! ```ignore
//! // tree structure:
//! // 0 -- 1 -- 2
//! //  \
//! //   3 -- 4 -- 5
//!
//! let root = Stack::root(0);
//! let two  = root.push(1).push(2);
//! let five = root.push(3).push(4).push(5);
//!
//! assert_eq!(*two, 2);
//! assert_eq!(*five, 5);
//! ```
//! Crate Content
//!
//! This crate provides two stack implementations:
//! [`Stack`](struct.Stack.html) and [`VStack`](struct.VStack.html). In short:
//! `Stack` uses a recursive (pointer) architecture, whilst `VStackc` uses a
//! vector to store the stack's data.
//!

use std::ops::Deref;
use std::rc::Rc;


struct StackNode<T> {
    value: T,
    parent: Option<Stack<T>>,
}


/// Recursive Stack
///
/// A **recursive** implementation for ``Stack``.
///
/// ```rust,ignore
/// pub struct StackNode<T> {
///     value: T,
///     parent: Option<Stack<T>>,
/// }
/// ```
///
pub struct Stack<T>(Rc<StackNode<T>>);

impl<T> Clone for Stack<T> {
    fn clone(&self) -> Self {
        Stack(self.0.clone())
    }
}

impl<T> Stack<T> {

    pub fn root(val: T) -> Self {
        Stack(Rc::from(StackNode {
            value: val,
            parent: None,
        }))
    }

    pub fn push(&self, val: T) -> Self {
        Stack(Rc::from(StackNode {
            value: val,
            parent: Some(Stack(self.0.clone())),
        }))
    }

    pub fn pop(&self) -> Option<Self> {
        match self.0.parent {
            None => None,
            Some(ref p) => Some(p.clone()),
        }
    }
}

impl<T> Deref for Stack<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0.deref().value
    }
}
