

extern crate kaktus;

use kaktus::{PushPop, Stack};

#[test]
fn basic() {
    let three = kaktus::Stack::root(1).push(2).push(3);
    let two = three.pop().unwrap();
    let one = two.pop().unwrap();

    assert_eq!(*three, 3);
    assert_eq!(*two, 2);
    assert_eq!(*one, 1);
}

#[test]
fn option() {
    let empty_stack = Stack::empty();
    let one = empty_stack.push(1);

    assert_eq!(empty_stack.peek(), None);
    assert_eq!(one.peek(), Some(&1));
}


#[test]
fn split() {
    let root = Stack::root(1);
    let a = root.push(2).push(4);
    let b = root.push(3).push(5);

    assert_eq!(
        *a.pop().unwrap().pop().unwrap(),
        *b.pop().unwrap().pop().unwrap()
    );
}


#[test]
fn depth() {
    let empty_stack = Stack::empty();

    assert!(empty_stack.is_none());
    assert_eq!(empty_stack.depth(), 0);

    let one = empty_stack.push(1);
    assert_eq!(one.depth(), 1);
}
