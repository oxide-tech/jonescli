use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::node::ContextNode;
use super::rules::LogicContext;
use super::state::ContextState;
use super::types::{Indent, ContextType};
use super::{ALL_MATCH, CLASS_MATCH, DEF_MATCH, DOCSTRING_END, DOCSTRING_START};

pub struct ContextProcessor {
    pub context_state: ContextState,
    pub line_counter: usize,
    pub file_lines: Vec<String>,
    pub(super) rules: LogicContext,
    pub(super) max_height: usize,
    pub(super) indent: Indent,
    pub(super) patterns: HashMap<ContextType, Regex>,
}

impl ContextProcessor {
    pub fn load(file_lines: Vec<String>) -> Self {
        let mut patterns: HashMap<ContextType, Regex> = HashMap::new();
        patterns.insert(
            ContextType::METHOD,
            Regex::new(DEF_MATCH).unwrap(),
        );
        patterns.insert(
            ContextType::CLASS,
            Regex::new(CLASS_MATCH).unwrap(),
        );
        patterns.insert(
            ContextType::ALL,
            Regex::new(ALL_MATCH).unwrap(),
        );
        patterns.insert(
            ContextType::DOCSTRING,
            Regex::new(DOCSTRING_START).unwrap(),
        );

        Self {
            context_state: ContextState::new(),
            line_counter: 0,
            max_height: file_lines.len(),
            file_lines,
            patterns,
            indent: Indent::new(),
            rules: LogicContext::new()
        }
    }

    /// Checks for context entry point.
    ///
    /// If a line matches a python definition such as a class/method/all it
    /// returns the specific context.
    pub(super) fn check_context_entry(
        &self,
        current_line: &String,
    ) -> Option<ContextType> {
        for (context_type, pattern) in &self.patterns {
            if pattern.is_match(current_line) {
                if self.rules.contains(&self.context_state.context_type)
                    && self.rules.contains(context_type)
                {
                    continue;
                }
                return Some(context_type.to_owned());
            }
        }

        None
    }

    /// Changes the state of the processor so that it reflects being inside a
    /// context.
    pub(super) fn start_context(
        &mut self,
        context_type: ContextType,
        current_line: &String,
    ) {
        self.context_state.context_type = context_type.clone();
        let context_name = self.get_context_name(current_line);
        let is_public = !context_name.starts_with("_");
        let child_node = ContextNode::new(
            context_name,
            context_type.clone(),
            self.line_counter,
            is_public,
        );
        self.context_state.descend(child_node);
        if !self.rules.contains(&self.context_state.context_type) {
            self.indent.increase();
        }
    }

    /// Extracts context name based on context type
    pub(super) fn get_context_name(&self, current_line: &String) -> String {
        match self.context_state.context_type {
            ContextType::METHOD => {
                let c = self
                    .patterns
                    .get(&ContextType::METHOD)
                    .unwrap()
                    .captures(current_line)
                    .unwrap();
                c.get(0).unwrap().as_str().to_string()
            }
            ContextType::CLASS => {
                let pattern = self.patterns.get(&ContextType::CLASS).unwrap();
                let c = pattern.captures(current_line).unwrap();
                c.get(0).unwrap().as_str().to_string()
            }
            _ => String::from("__empty__"),
        }
    }

    /// Check if the current line represents an exit point from the
    /// current context.
    pub(super) fn check_context_exit(&self, current_line: &String) -> bool {
        let line_is_empty = |line: &String| line.is_empty() || !line.starts_with("    ");
        let get_second_line = |line_counter: usize| &self.file_lines[line_counter + 1];

        match self.context_state.context_type {
            ContextType::ROOT => false,
            ContextType::DOCSTRING => {
                let r = Regex::new(DOCSTRING_END).unwrap();
                r.is_match(current_line)
            }
            ContextType::CLASS => {
                if !current_line.is_empty() {
                    return false;
                }
                if self.line_counter + 1 > self.max_height { true }
                else if line_is_empty(&get_second_line(self.line_counter)) { true }
                else { false }
            }
            _ => {
                if !current_line.is_empty() {
                    return false;
                }
                let second_line = get_second_line(self.line_counter);
                if self.line_counter + 1 > self.max_height { true }
                else if line_is_empty(&second_line) { true }
                else { false }
            }
        }
    }

    /// Changes the state of the processor so that it reflects being outside
    /// current context.
    fn exit_context(&mut self) -> () {
        self.indent.decrease();
        self.context_state.ascend();
        self.context_state
            .context_node
            .borrow_mut()
            .set_location(self.line_counter);
    }

    /// Extracting lines that are used inside unique context types
    ///
    /// For example, docstring context would need the string values.
    /// Each new context can be safely added as a match arm to this method.
    fn extract_context_data(&mut self, current_line: &String) {
        match self.context_state.context_type {
            ContextType::DOCSTRING => {
                self.context_state
                    .context_node
                    .borrow_mut()
                    .append_value(current_line);
            }
            _ => (),
        }
    }

    fn check_root_exit(&mut self) -> bool {
        if self.line_counter + 1 >= self.max_height {
            return false;
        }
        let second_line = &self.file_lines[self.line_counter + 1];
        if !second_line.starts_with(self.indent.value()) {
            while self.context_state.context_type != ContextType::ROOT {
                self.exit_context();
            }
            return true;
        }
        false
    }

    pub fn parse_module(&mut self) -> Rc<RefCell<ContextNode>> {
        let mut lines = self
            .file_lines
            .to_owned()
            .into_iter()
            .take(self.max_height - 1);

        loop {
            // Fetch current line
            let current_line = match lines.next() {
                Some(line) => line,
                None => {
                    self.context_state.top();
                    break;
                }
            };

            // Check context entry
            if let Some(c) = self.check_context_entry(&current_line) {
                self.start_context(c, &current_line);
            }

            // Process any kind of context for values
            self.extract_context_data(&current_line);

            // Check exit

            if self.check_context_exit(&current_line) {
                if !self.check_root_exit() {
                    self.exit_context();
                }
            }

            // Increment line counter
            self.line_counter += 1;
        }

        Rc::clone(&self.context_state.context_node)
    }
}
