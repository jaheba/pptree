

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

type Tree<T> = Rc<RefCell<PPTree<T>>>;


pub struct NodeData<T> {
    value: T,
    tree: *const RefCell<PPTree<T>>,
    position: usize,
    parent: Option<usize>,
}

impl<T> NodeData<T> {
    fn new(val: T, tree: *const RefCell<PPTree<T>>, position: usize, parent: Option<usize>) -> Self {
        NodeData { value: val, tree: tree, position: position, parent: parent }
    }

    fn tree(&self) -> &RefCell<PPTree<T>> {
        unsafe {
            &*self.tree
        }
    }

    fn push(&self, val: T) -> usize {
        let new = NodeData::new(val, self.tree, 0, Some(self.position));
        self.tree().borrow_mut().push(new)
    }
}

impl<T> Deref for NodeData<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

struct PPTree<T> {
    data: Vec<NodeData<T>>,
}

impl<T> PPTree<T> {
    fn new() -> Self {
        PPTree { data: Vec::new() }
    }

    fn push(&mut self, mut node: NodeData<T>) -> usize {
        let pos = self.data.len();
        node.position = pos;
        self.data.push(node);
        pos
    }
}

#[derive(Clone)]
pub struct Node<T> {
    tree: Tree<T>,
    idx: usize,
}

impl<T> Node<T> {
    fn deref_node(&self) -> &NodeData<T> {
        let raw_ptr = &*self.tree.borrow() as *const PPTree<T>;
        let tree = unsafe {
            &*raw_ptr
        };
        &tree.data[self.idx]
    }

    pub fn root(val: T) -> Node<T> {
        let tree = Rc::from(RefCell::from(PPTree::new()));
        let this = NodeData::new(val, tree.deref(), 0, None);
        let idx = tree.borrow_mut().push(this);
        Node { tree: tree, idx: idx }
    }

    pub fn push(&self, val: T) -> Self {
        let node = self.deref_node();
        let new_pos = node.push(val);
        Node { tree: self.tree.clone(), idx: new_pos }
    }

    pub fn pop(&self) -> Option<Self> {
        let node = self.deref_node();
        node.parent.map(|idx| Node { tree: self.tree.clone(), idx: idx} )
    }
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.deref_node().value
    }
}


#[test]
fn test() {
    let x = {
        let root = Node::root(0);
        let one = root.push(42);
        let x = root.push(1).push(2).push(3).push(4).push(5).push(6);
        let val = *root;
        assert_eq!(val, 0);
        one
    };

    assert_eq!(*x, 42);
    assert_eq!(*x.pop().unwrap(), 0);
    x.push(13);
}
