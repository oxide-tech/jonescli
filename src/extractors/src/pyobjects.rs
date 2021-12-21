use std::fmt;
use ansi_term::Colour;

/// Parameter of a python method
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Parameter{

    /// The name of the parameter
    pub name: String,

    // The [optional] typing this parameter has
    pub static_type: String
}
impl Parameter {
    pub fn new(name: String, static_type: String) -> Self { Parameter { name, static_type } }
}
impl fmt::Display for Parameter{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "  * {}: {}",
            Colour::Purple.paint(&self.name),
            Colour::Green.paint(&self.static_type)
        )
    }
}

/// Representation of a python class method
/// > NOTE: Not a @classmethod but a method found in a initialized class
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Method{

    /// Method name
    pub name: String,

    /// Parameters of the method
    pub parameters: Vec<Parameter>,

    /// The output type of a method
    pub output: String
}
impl Method {
    pub fn new(name: String, parameters: Vec<Parameter>, output: String) -> Self {
        Method { name, parameters, output }
    }
}
impl fmt::Display for Method{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, ":: [{}] -> {}",
            Colour::Yellow.paint(&self.name),
            Colour::Cyan.paint(&self.output)
        )
    }
}

/// Representation of the Python Class
#[derive(Debug)]
#[derive(PartialEq)]
pub struct PythonClass{
    pub name: String,
    pub methods: Vec<Method>,
    pub inheritance: Vec<String>,
    pub docstring: String
}
impl PythonClass {
    pub fn new(
        name: String,
        methods: Vec<Method>,
        inheritance: Vec<String>,
        docstring: String
    ) -> Self {
        PythonClass { name, methods, inheritance, docstring }
    }
}
impl fmt::Display for PythonClass{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let inheritance_display = self.inheritance.join(", ");
        write!(f, "# Class :: [{}]\n{}\n* inherit -> {}\n\n# Methods\n-------",
            Colour::Cyan.paint(&self.name),
            Colour::Yellow.paint(&self.docstring),
            Colour::Green.paint(inheritance_display)
        )
    }
}