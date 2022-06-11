pub type CodeLocation = (usize, usize);

/// NamespaceType refers to a code block context type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContextType {
    /// Default namespace
    ROOT,

    /// Represents a python namespace context
    METHOD,

    /// Represents a python namespace context
    CLASS,

    /// Represents a __all__ namespace context
    ALL,

    /// Represents a docstring context
    DOCSTRING,
}

pub struct Indent {
    value: String,
    spaces: usize
}
impl Indent {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            spaces: 0
        }
    }

    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn increase(&mut self) {
        self.spaces += 4;
        self.value.push_str("    ");
    }

    pub fn decrease(&mut self) {
        if self.spaces > 0 {
            self.spaces -= 4;
            self.value = self.value.chars().take(self.spaces).collect();
        }
    }
}