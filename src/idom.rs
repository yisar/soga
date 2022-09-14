use std::collections::HashMap;
use crate::green::*;
use crate::red::*;

struct Idom {
    current_node: Option<RedTree>,
    current_parent: Option<RedTree>,
}

struct NodeData {
    name: String,
    text: String,
}

impl Idom {
    fn new() -> Idom {
        Idom {
            current_node: None,
            current_parent: None,
        }
    }

    fn next(&mut self) -> Option<RedTree> {
        match &self.current_node {
            Some(node) => node.next_sibling(),
            None => None, // first_child
        }
    }

    fn enter(&mut self) {
        match &self.current_node {
            Some(node) => {
                self.current_parent = Some(node.clone());
            }
            None => {
                self.current_parent = None;
            }
        }

        self.current_node = None;
    }

    // fn exit(&mut self) {
    //     self.current_node = self.current_parent;
    //     self.current_parent = match self.current_node {
    //         Some(node) => { node.parent() }
    //         None => None,
    //     };
    // }

    fn matches(&mut self, node: &RedTree, name: &String) -> bool {
        true
    }

    fn render(&mut self, name: &String) -> Option<RedTree> {
        if let Some(node) = &mut self.current_node {
            let mut green = GreenTree::new(name, 0, 0).into();
            let mut red = RedTree::new(green, None, 0);
            node.insert_child(0, red.clone());
            Some(red);
        }
        None
    }

    fn open_start(&mut self, name: &String) {
        self.next();
        self.render(name);
    }

    fn open_close(&mut self, name: &String) {
        self.enter();
    }
}

impl Idom {
    fn open(&mut self, name: String) {
        self.open_start(&name);
        self.open_close(&name);
    }
}