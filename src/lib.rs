
use std::cell::{Ref, RefCell, RefMut};



pub struct Node<T> {
    nodes: *mut Tree<T>,
    parent: Option<*const Node<T>>,
    value: RefCell<T>,
}


impl<T> Node<T> {
    fn new(nodes: *mut Tree<T>, parent: Option<*const Node<T>>, value: T) -> Self {
        Node {
            nodes: nodes,
            parent: parent,
            value: RefCell::from(value),
        }
    }

    pub fn push(&self, val: T) -> &Node<T> {
        let node = Node::new(self.nodes, Some(self), val);
        unsafe {
            let nodes = &mut *self.nodes;
            nodes.values.push(node);
            nodes.values.last().unwrap()
        }
    }

    pub fn borrow(&self) -> Ref<T> {
        self.value.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.value.borrow_mut()
    }

    pub fn parent(&self) -> Option<&Node<T>> {
        unsafe { self.parent.map(|p| &*p) }
    }

    pub fn iter(&self) -> NodeIter<T> {
        NodeIter { node: Some(self) }
    }
}


pub struct NodeIter<'a, T: 'a> {
    node: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for NodeIter<'a, T> {
    type Item = Ref<'a, T>;

    fn next(&mut self) -> Option<Ref<'a, T>> {
        self.node.map(|n| {
            self.node = n.parent();
            n.value.borrow()
        })
    }
}


struct Tree<T> {
    values: Vec<Node<T>>,
}

impl<T> Tree<T> {
    fn new() -> Self {
        Tree { values: Vec::new() }
    }
}


pub struct PPTree<T> {
    tree: Box<Tree<T>>,
}

impl<T> PPTree<T> {
    pub fn new(val: T) -> Self {
        let mut tree = Box::from(Tree::new());
        let node = Node::new(&mut *tree, None, val);
        tree.values.push(node);
        PPTree { tree: tree }
    }

    pub fn root(&self) -> &Node<T> {
        &self.tree.values[0]
    }
}


#[test]
fn test() {
    let tree = PPTree::new(0);
    let root = tree.root();
    let x = root.push(1).push(2).push(3);
    let y = root.push(2).push(5);

    *y.borrow_mut() = 6;

    for v in x.iter() {
        println!("{:?}", v);
    }

    println!("{:?}", y.iter().collect::<Vec<_>>());
}
