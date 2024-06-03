use std::{cell::RefCell, collections::HashMap, mem::discriminant, ops::Deref, rc::{Rc, Weak}};

use crate::dom::DomError;


#[derive(Debug)]
pub enum NodeType {
    Text(RefCell<String>),
    Element
}

#[derive(Debug)]
pub struct Node {
    pub tag_name: String,
    pub children: Rc<RefCell<Vec<Rc<RefCell<Node>>>>>,
    pub parent: Option<Weak<RefCell<Node>>>,
    pub node_type: NodeType,
    pub attributes: Rc<RefCell<HashMap<String, String>>>
}

impl Node {
    pub fn new(tag_name: String) -> Self {
        Self {
            tag_name,
            children: Rc::new(RefCell::new(vec![])),
            parent: None,
            node_type: NodeType::Text(RefCell::new("".into())),
            attributes: Rc::new(RefCell::new(HashMap::new()))
        }
    }

    pub fn children(&self) -> Rc<RefCell<Vec<Rc<RefCell<Node>>>>> {
        self.children.clone()
    }

    pub fn parent(&self) -> Option<Weak<RefCell<Node>>> {
        match &self.parent {
            Some(p) => Some(p.clone()),
            None => None,
        }
    }

    pub fn get_element_by_attribute(&self, key: &str, value: &str) -> Option<Rc<RefCell<Node>>> {
        for child in self.children.deref().borrow().iter() {
            if let Some(val) = child.deref().borrow().attributes.borrow().get(key) {
                if val == value {
                    return Some(child.clone());
                }
            }
        }
        None
    }

    pub fn get_elements_by_attribute(&self, key: &str, value: &str) -> Vec<Rc<RefCell<Node>>> {
        let mut elements = vec![];
        for child in self.children.deref().borrow().iter() {
            if let Some(val) = child.deref().borrow().attributes.borrow().get(key) {
                if val == value {
                    elements.push(child.clone());
                }
            }
        }
        elements
    }

    pub fn append_element(parent: &Rc<RefCell<Node>>, mut node: Node) ->  Result<(), DomError> {
        if discriminant(&parent.deref().borrow().node_type) == discriminant(&NodeType::Text(RefCell::new("".into()))) {
            Err(DomError::BlockedAppend("attempt to append element to a text element".into()))
        } else {
            node.parent = Some(Rc::downgrade(&parent));
            parent.deref().borrow().children.borrow_mut().push(Rc::new(RefCell::new(node)));
            Ok(())
        }
    }

    pub fn append_text(&mut self, string: &str) -> Result<(), DomError> {
        match &self.node_type {
            NodeType::Element => {
                return Err(DomError::BlockedAppend("attempt to append text on element node".into()));
            },
            NodeType::Text(t) => {
                t.borrow_mut().push_str(string);
                Ok(())
            }
        }
    }

    fn get_attributes(&mut self) -> Rc<RefCell<HashMap<String, String>>> {
        self.attributes.clone()
    }
}

impl ToString for Node {
    fn to_string(&self) -> String {
        let mut attribute_string = String::new();
        for (key, value) in self.attributes.borrow().iter() {
            attribute_string.push_str(format!("{key}='{value}'").as_str());
        }

        if self.children.deref().borrow().len() > 0 {
            let mut children_string = String::new();
            for child in self.children.deref().borrow().iter() {
                children_string.push_str(&child.deref().borrow().to_string())
            }
            format!("<{0}>{1}</{0}>", self.tag_name, children_string)
        } else {
            if let NodeType::Text(txt) = &self.node_type {
                format!("<{0} {1}>{2}</{0}>", self.tag_name, attribute_string, txt.borrow())
            } else {
                format!("<{} {}/>", self.tag_name, attribute_string)
            }
        }
    }
}