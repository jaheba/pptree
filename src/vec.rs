
use std::rc::Rc;
use std::cell::{RefCell, Ref};
use std::ops::Deref;

use super::Stack;

/// Newtype for `Vec<Node<T>>`.
struct Tree<T> {
    data: Vec<Node<T>>,
}

impl<T> Tree<T> {
    fn new() -> Self {
        Tree { data: Vec::new() }
    }

    fn push(&mut self, node: Node<T>) -> usize {
        let pos = self.data.len();
        self.data.push(node);
        pos
    }
}


/// Node
struct Node<T> {
    value: T,
    parent: Option<usize>,
}

impl<T> Node<T> {
    fn new(val: T,
           parent: Option<usize>)
           -> Self {
        Node {
            value: val,
            parent: parent,
        }
    }

    // fn push(&self, val: T) -> usize {
    //     let new = Node::new(val, self.tree, 0, Some(self.position));
    //     self.tree().borrow_mut().push(new)
    // }
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

// fn elide<'a, T>(reference: &T) -> &'a T {
//     unsafe {
//         &*(reference as *const T)
//     }
// }

#[derive(Clone)]
pub struct VStack<T> {
    tree: Rc<RefCell<Tree<T>>>,
    idx: usize,
}

struct StackRef<'a, T: 'a> {
    r: Ref<'a, Tree<T>>,
    idx: usize,
}

impl<'a, T> StackRef<'a, T> {
    fn deref_node(&self) -> &Node<T> {
        &self.r.data[self.idx]
    }
}

impl<'a, T> Deref for StackRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.deref_node().value
    }
}

fn elide<'a, T>(reference: &T) -> &'a T {
    unsafe {
        &*(reference as *const T)
    }
}

impl<T> VStack<T> {
    unsafe fn deref(&self) -> &T {
        elide(&self.tree.borrow().data[self.idx])
    }

    fn get<'a>(&'a self) -> StackRef<'a, T> {
        // elide(&self.tree.borrow().data[self.idx])
        StackRef { r: self.tree.borrow(), idx: self.idx }
    }

    fn try_get<'a>(&'a self) -> Option<StackRef<'a, T>> {
        match self.tree.try_borrow() {
            Ok(r) => Some(StackRef { r: r, idx: self.idx }),
            _ => None,
        }
    }

    fn try_push(&mut self, val: T) -> Option<Self> {
        {
            if let Err(_) = self.tree.try_borrow_mut() {
                return None
            }
        }

        Some(self.push(val))
    }
}

impl<T> Stack<T> for VStack<T> {

    /// Constructs a new `VStack` with `val` as its root-node.
    fn root(val: T) -> VStack<T> {
        let tree = Rc::from(RefCell::from(Tree::new()));
        let node = Node::new(val, None);
        let idx = tree.borrow_mut().push(node);
        VStack {
            tree: tree,
            idx: idx,
        }
    }

    fn push(&self, val: T) -> Self {
        let new = Node::new(val, Some(self.idx));
        let new_pos = self.tree.borrow_mut().push(new);
        VStack {
            tree: self.tree.clone(),
            idx: new_pos,
        }
    }

    fn pop(&self) -> Option<Self> {
        let node = self.get();
        node.deref_node().parent.map(|idx| {
            VStack {
                tree: self.tree.clone(),
                idx: idx,
            }
        })
    }
}


#[test]
fn test() {
    let mut root = VStack::root(0);
    let mut two = root.try_push(2).unwrap();
    // {
    let x = two.try_get();
    // }

    match root.try_push(2) {
        None => println!("OK"),
        _ => println!("not good"),
    }

    unsafe {
        println!("{:?}", two.deref());
    }
}
