
use std::rc::Rc;
use std::cell::RefCell;
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

    /// Push `Node` to vector.
    /// Sets the index inside the node to allow spawning child-nodes from node.
    fn push(&mut self, mut node: Node<T>) -> usize {
        let pos = self.data.len();
        node.position = pos;
        self.data.push(node);
        pos
    }
}


/// Node
struct Node<T> {
    value: T,
    tree: *const RefCell<Tree<T>>,
    position: usize,
    parent: Option<usize>,
}

impl<T> Node<T> {
    fn new(val: T,
           tree: *const RefCell<Tree<T>>,
           position: usize,
           parent: Option<usize>)
           -> Self {
        Node {
            value: val,
            tree: tree,
            position: position,
            parent: parent,
        }
    }

    fn tree(&self) -> &RefCell<Tree<T>> {
        unsafe { &*self.tree }
    }

    fn push(&self, val: T) -> usize {
        let new = Node::new(val, self.tree, 0, Some(self.position));
        self.tree().borrow_mut().push(new)
    }
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}



#[derive(Clone)]
pub struct VStack<T> {
    tree: Rc<RefCell<Tree<T>>>,
    idx: usize,
}


impl<T> VStack<T> {
    fn deref_node(&self) -> &Node<T> {
        let raw_ptr = &*self.tree.borrow() as *const Tree<T>;
        let tree = unsafe { &*raw_ptr };
        &tree.data[self.idx]
    }

}

impl<T> Stack<T> for VStack<T> {
    /// Constructs a new `VStack` with `val` as its root-node.
    fn root(val: T) -> VStack<T> {
        let tree = Rc::from(RefCell::from(Tree::new()));
        let this = Node::new(val, tree.deref(), 0, None);
        let idx = tree.borrow_mut().push(this);
        VStack {
            tree: tree,
            idx: idx,
        }
    }

    fn push(&self, val: T) -> Self {
        let node = self.deref_node();
        let new_pos = node.push(val);
        VStack {
            tree: self.tree.clone(),
            idx: new_pos,
        }
    }

    fn pop(&self) -> Option<Self> {
        let node = self.deref_node();
        node.parent.map(|idx| {
            VStack {
                tree: self.tree.clone(),
                idx: idx,
            }
        })
    }
}

impl<T> Deref for VStack<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.deref_node().value
    }
}
