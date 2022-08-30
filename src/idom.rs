use std::collections::HashMap;
use crate::green::*;
use crate::red::*;

struct Idom {
    current_node: Option<GreenTree>,
    current_parent: Option<GreenTree>,
    attr_builder: HashMap<String, String>,
}

struct NodeData {
    name: String,
    text: String,
    attrs: HashMap<String, String>,
}

impl Idom {
    fn new() {
        Idom {
            current_node: None,
            current_parent: None,
            attr_builder: HashMap::new(),
        }
    }

    fn next(&mut self) -> RedTree {
        match self.current_node {
            Some(node) => node.next(),
            None => node.first(),
        }
    }

    fn enter(&mut self) -> Void {
        self.current_parent = Some(self.current_node);
        self.current_node = Node;
    }

    fn exit(&mut self) -> Void {
        self.current_node = self.current_parent;
        self.current_parent = self.current_parent.parent();
    }

    fn matches(&mut self, node: RedTree, name: String) -> bool {
        let data = self.get_data(node);
        return data.name == name;
    }

    fn render(&mut self, name: String) -> Option<RedTree> {
        if self.current_node && self.matches(self.current_node, name) {
            return self.current_node;
        }

        let mut node = GreenTree::new(name, 0, 0);

        self.current_node.insert_child(node.index(), self.current_node.index())
    }

    fn open_start(&mut self, name: String) -> Void {
        self.next();
        self.render(name);
    }

    fn open_close(&mut self, name: String) -> Void {
        let data = self.get_data(self.current_node);
        let prev_props = data.attrs;

        // diff props

        self.attr_builder = None;
        self.enter();
    }

    fn get_data(&mut self, node) -> NodeData {
        match node.data {
            None => {
                let data = NodeData {
                    name: node.tag(),
                    text: String::new(),
                    attrs: HashMap::new(),
                };
                node.data = Some(data);
                return node.data;
            }
            Some(data) => {
                return data;
            }
        }
    }

}

impl Idom{
    fn text(&mut self, value: String) -> Void {
        self.next();
        let node = self.render("#text".to_string());
        let data = self.get_data(node);

        if data.text != value {
            data.text = value;
            node.data = value;
        }
    }

    fn close(&mut self, name: String) -> Void {
        let next_node = self.next();
        loop {
            if next_node == None {
                break;
            }
            let next = next_node.next();
            self.current_parent.remove_child(next_node.index());
            next_node = next;
        }
        exit()
    }


    fn open(&mut self, name: String, attrs: HashMap<String, String>) -> Void {
        self.open_start(name);
        self.attr_builder = attrs;
        self.close_start(name);
    }
}