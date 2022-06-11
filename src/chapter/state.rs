use std::cell::RefCell;
use std::rc::Rc;

use super::node::ContextNode;
use super::types::ContextType;

pub struct ContextState {
    /// Current context type
    pub context_type: ContextType,

    /// Current context node
    pub context_node: Rc<RefCell<ContextNode>>,
}

impl ContextState {
    pub fn new() -> Self {
        Self {
            context_type: ContextType::ROOT,
            context_node: ContextNode::root(),
        }
    }

    /// Adding the new_node as child to the current node and referencing
    /// the current node as parent to the new node
    ///
    /// Descending into the context tree
    pub fn descend(&mut self, new_node: Rc<RefCell<ContextNode>>) {
        // Add new node as child to current node
        self.context_node
            .borrow_mut()
            .add_node(Rc::clone(&new_node));

        // Reference current node as parent to new node
        new_node
            .borrow_mut()
            .set_parent(Rc::clone(&self.context_node));

        // Set the current node to the new node
        self.context_node = new_node;
        self.context_type = self.context_node.borrow().context_type.to_owned();
    }

    /// Ascending to the parent node of the current node
    pub fn ascend(&mut self) {
        let parent = match self.context_node.borrow_mut().parent.to_owned() {
            Some(v) => v,
            None => ContextNode::root(),
        };

        self.context_node = parent;
        self.context_type = self.context_node.borrow().context_type.to_owned();
    }

    pub fn top(&mut self) {
        while self.context_type != ContextType::ROOT {
            self.ascend();
        }
    }
}