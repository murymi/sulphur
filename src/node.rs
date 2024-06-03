use std::{cell::RefCell, collections::HashMap, rc::{Rc, Weak}};


#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element
}

#[derive(Debug)]
pub struct Node {
    pub tag_name: String,
    pub children: Vec<Rc<RefCell<Node>>>,
    pub parent: Option<Weak<RefCell<Node>>>,
    pub node_type: NodeType,
    pub attributes: HashMap<String, String>
}

impl Node {
    pub fn new(tag_name: String) -> Self {
        Self {
            tag_name,
            children: vec![],
            parent: None,
            node_type: NodeType::Text("".into()),
            attributes: HashMap::new()
        }
    }
}

impl ToString for Node {
    fn to_string(&self) -> String {
        let mut attribute_string = String::new();
        for (key, value) in self.attributes.iter() {
            attribute_string.push_str(format!("{key}=\"{value}\"").as_str());
        }

        if self.children.len() > 0 {
            let mut children_string = String::new();
            for child in self.children.iter() {
                children_string.push_str(&child.borrow().to_string())
            }
            format!("<{0}>{1}</{0}>", self.tag_name, children_string)
        } else {
            if let NodeType::Text(txt) = &self.node_type {
                format!("<{0} {1}>{2}</{0}>", self.tag_name, attribute_string, txt)
            } else {
                format!("<{} {}/>", self.tag_name, attribute_string)
            }
        }
    }
}