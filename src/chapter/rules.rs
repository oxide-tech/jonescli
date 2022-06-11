use super::types::ContextType;

/// Contains a global vector of context types that should be omitted when
/// walking through the definitions of a py module.
///
/// For example: Docstrings do not represent a specific defined context like
/// a class or method. Exiting from a docstring should not lead to the same
/// rules as exiting from methods or classes
pub struct LogicContext {
    contexts: Vec<ContextType>
}

impl LogicContext {
    pub fn new() -> Self {

        // NOTE: Add here all contexts that have logic or values in it
        // to be omitted when walking through definitions
        let contexts = vec![
            ContextType::DOCSTRING
        ];

        Self { contexts}
    }

    pub fn contains(&self, context: &ContextType) -> bool{
        self.contexts.contains(context)
    }
}