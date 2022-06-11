use std::cell::RefCell;
use std::rc::Rc;

use super::types::{ContextType, CodeLocation};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ContextNode {
    pub name: String,
    pub context_type: ContextType,
    start: usize,
    end: usize,
    pub is_public: bool,
    pub value: Option<String>,
    pub children: Vec<Rc<RefCell<ContextNode>>>,
    pub parent: Option<Rc<RefCell<ContextNode>>>
}

impl ContextNode {
    pub fn add_node(&mut self, child_node: Rc<RefCell<ContextNode>>) {
        self.children.push(child_node);
    }

    pub fn append_value(&mut self, new_value: &String) {
        match self.value.to_owned() {
            Some(mut v) => v.push_str(new_value),
            None => self.value = Some(new_value.to_owned())
        }
    }

    pub fn location(&self) -> Option<CodeLocation>{
        if self.end != usize::MIN {
            return Some((self.start, self.end))
        }
        None
    }

    pub fn new(name: String, context_type: ContextType, start: usize, is_public: bool) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ContextNode {
            name,
            context_type,
            start,
            end: usize::MIN,
            is_public,
            value: None,
            children: Vec::new(),
            parent: None
        }))
    }

    pub fn root() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ContextNode {
            name: String::from("__root__"),
            context_type: ContextType::ROOT,
            start: usize::MIN,
            end: usize::MIN,
            is_public: false,
            value: None,
            children: Vec::new(),
            parent: None
        }))
    }

    pub fn set_location(&mut self, end: usize) {
        if self.end == usize::MIN {
            self.end = end
        }
    }

    pub fn set_parent(&mut self, parent_node: Rc<RefCell<ContextNode>>) {
        self.parent = Some(parent_node);
    }
}