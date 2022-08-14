use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Arena<T, K> {
    root: Option<usize>,
    nodes: Vec<Node<T, K>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Node<T, K> {
    parent: Option<usize>,
    previous_sibling: Option<usize>,
    next_sibling: Option<usize>,
    first_child: Option<usize>,
    last_child: Option<usize>,

    pub key: K,
    pub data: T,
}

impl<T, K> Arena<T, K>
where
    K: PartialEq,
{
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            root: None,
        }
    }

    pub fn insert(&mut self, data: T, key: K, parent: Option<usize>) -> usize {
        let next_index = self.nodes.len();

        let mut node = Node::new(data, key);
        node.set_parent(parent);
        self.nodes.push(node);

        next_index
    }

    pub fn root(&self) -> Option<usize> {
        self.root
    }

    pub fn set_root(&mut self, new_root: Option<usize>) {
        self.root = new_root;
    }

    pub fn nodes(&self) -> &Vec<Node<T, K>> {
        &self.nodes
    }

    pub fn find_inner(&self, key: K) -> Option<&T> {
        for node in self.nodes.iter() {
            if node.key == key {
                return Some(&node.data);
            }
        }

        None
    }

    pub fn get_inner(&self, index: usize) -> Option<&T> {
        let node = self.nodes.get(index);

        if let Some(node) = node {
            Some(&node.data)
        } else {
            None
        }
    }

    pub fn get_inner_mut(&mut self, index: usize) -> Option<&mut T> {
        let node = self.nodes.get_mut(index);

        if let Some(node) = node {
            Some(&mut node.data)
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<&Node<T, K>> {
        self.nodes.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Node<T, K>> {
        self.nodes.get_mut(index)
    }

    pub fn find(&self, key: K) -> Option<&Node<T, K>> {
        for node in self.nodes.iter() {
            if node.key == key {
                return Some(node);
            }
        }

        None
    }

    pub fn get_parent(&self, index: usize) -> Option<&Node<T, K>> {
        let node = self.get(index);

        if let Some(node) = node {
            match node.parent() {
                Some(index) => self.get(index),
                None => None,
            }
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.len() == 0
    }

    pub fn count(&self) -> usize {
        self.nodes.len()
    }
}

impl<T, K> Node<T, K> {
    pub fn new(data: T, key: K) -> Self {
        Node {
            parent: None,
            previous_sibling: None,
            next_sibling: None,
            first_child: None,
            last_child: None,
            key,
            data,
        }
    }

    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn parent(&self) -> Option<usize> {
        self.parent
    }

    pub fn set_parent(&mut self, new_parent: Option<usize>) {
        self.parent = new_parent
    }

    pub fn previous_sibling(&self) -> Option<usize> {
        self.previous_sibling
    }

    pub fn next_sibling(&self) -> Option<usize> {
        self.next_sibling
    }

    pub fn first_child(&self) -> Option<usize> {
        self.first_child
    }

    pub fn last_child(&self) -> Option<usize> {
        self.last_child
    }
}


// fn main(){
//     let arena:Arena<usize, usize> = Arena::new();
//     println!("{:#?}",arena);
// }