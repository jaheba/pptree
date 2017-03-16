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
use std::fmt::{self, Debug};
use std::iter::{IntoIterator, Iterator};


pub trait PushPop<T>
    where Self: std::marker::Sized
{
    type This;
    fn push(&self, val: T) -> Self::This;
    fn pop(&self) -> Option<Self::This>;
    fn peek(&self) -> Option<&T>;

    fn walk(&self) -> StackIterator<T>;

    fn depth(&self) -> usize {
        self.walk().count()
    }
}

struct Cell<T>
    where T: Sized
{
    value: T,
    parent: Option<Rc<Cell<T>>>,
}

impl<T> Cell<T> {
    fn orphan(val: T) -> Rc<Self> {
        Cell {
                value: val,
                parent: None,
            }
            .into()
    }

    fn with_parent(val: T, parent: Rc<Cell<T>>) -> Rc<Self> {
        Cell {
                value: val,
                parent: Some(parent),
            }
            .into()
    }
}

pub struct Stack<T> {
    cell: Rc<Cell<T>>,
}

impl<T> Stack<T> {
    pub fn empty() -> Option<Self> {
        None
    }

    pub fn root(val: T) -> Self {
        Stack::wrap(Cell::orphan(val))
    }

    fn wrap(cell: Rc<Cell<T>>) -> Self {
        Stack { cell: cell }
    }
}


impl<T> Clone for Stack<T> {
    fn clone(&self) -> Stack<T> {
        Stack::wrap(self.cell.clone())
    }
}


impl<T> PushPop<T> for Stack<T> {
    type This = Stack<T>;

    fn push(&self, val: T) -> Self {
        Stack { cell: Cell::with_parent(val, self.cell.clone()) }
    }

    fn pop(&self) -> Option<Self> {
        self.cell.parent.as_ref().cloned().map(Stack::wrap)
    }

    fn peek(&self) -> Option<&T> {
        Some(self.deref())
    }

    fn walk(&self) -> StackIterator<T> {
        self.into_iter()
    }
}

impl<T> PushPop<T> for Option<Stack<T>> {
    type This = Stack<T>;

    fn push(&self, val: T) -> Self::This {
        match *self {
            None => Stack::root(val),
            Some(ref stack) => stack.push(val),
        }
    }

    fn pop(&self) -> Self {
        self.as_ref().and_then(Stack::pop)
    }

    fn peek(&self) -> Option<&T> {
        self.as_ref().and_then(Stack::peek)
    }

    fn walk(&self) -> StackIterator<T> {
        StackIterator { current: self.as_ref().map(|stack| stack.clone()) }
    }
}

impl<T> Deref for Stack<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.cell.deref().value
    }
}

impl<T: Debug> Debug for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "S<{:?}>", **self)
    }
}

impl<'a, T> IntoIterator for &'a Stack<T> {
    type Item = Stack<T>;
    type IntoIter = StackIterator<T>;

    fn into_iter(self) -> StackIterator<T> {
        StackIterator { current: Some(self.clone()) }
    }
}

pub struct StackIterator<T> {
    current: Option<Stack<T>>,
}

impl<T> Iterator for StackIterator<T> {
    type Item = Stack<T>;

    fn next(&mut self) -> Option<Stack<T>> {
        let cur = self.current.take();
        self.current = cur.as_ref().and_then(Stack::pop);
        cur
    }
}
