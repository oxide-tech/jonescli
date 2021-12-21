use helpers::regex_split;

use crate::{CLASS_TEMPLATE_INHERITANCE, CLASS_TEMPLATE, TEMPLATE_KEYWORD};
use crate::pyobjects::PythonClass;
use crate::method::extract_methods;
use crate::docstring::extract_docstring;


/// Extract class inheritance objects
///
/// > Note: Does not include extracting filters or constraints
///
/// # Arguments
///
/// * `line` - Is the class header previously extracted
///
/// # Output
///
/// A vector containing all the objects the class inherits
pub fn extract_class_inheritance(line: &String) -> Option<Vec<String>> {

    // Split the header for now to get objects that are inherited by the class
    let class_header_split: Vec<&str> = regex_split(r"(\(|\):|,)", true, line);
    if class_header_split.len() == 1 {
        return None
    }

    // Iterate through the split results but ommit the first entry
    let mut class_inheritance: Vec<String> = Vec::new();
    for inheritance in class_header_split[1..class_header_split.len()-1].iter(){
        let inherit_object = inheritance.trim().to_string();
        class_inheritance.push(inherit_object);
    }

    Some(class_inheritance)
}

fn separate_code_block(code_lines: Vec<&str>, class_name: &str) -> (Vec<String>, String){
    let mut start_cutting: bool = false;
    let mut class_code_block: Vec<String> = Vec::new();
    let mut class_header: String = String::from("");
    let class_name_inheritance = CLASS_TEMPLATE_INHERITANCE
        .clone()
        .replace(TEMPLATE_KEYWORD, class_name);
    let class_name_simple = CLASS_TEMPLATE
        .clone()
        .replace(TEMPLATE_KEYWORD, class_name);


    for (counter, line) in code_lines.iter().enumerate() {
        if line.contains(&class_name_inheritance) || line.contains(&class_name_simple) {
            start_cutting = true;
            class_header.push_str(line)
        }
        if start_cutting {
            class_code_block.push(line.to_string());

            if line.len() <= 1 && code_lines[counter-1].len() <= 1 {
                break
            }
        }
    }

    (class_code_block, class_header)

}

/// Extracts the searched python class from the code
///
/// # Arguments
///
/// * `code_lines`: The full split into lines code file
///
/// * `class_name`: The searched class name
///
pub fn extract_python_class(code_lines: Vec<&str>, class_name: &str) -> PythonClass {
    let (class_code_block, class_header) = separate_code_block(code_lines, class_name);
    let class_methods = extract_methods(&class_code_block);
    let class_inheritance = match extract_class_inheritance(&class_header) {
        Some(inheritance_vec) => inheritance_vec,
        None => Vec::new()
    };
    let docstring = match extract_docstring(&class_code_block) {
        Some(inheritance_vec) => inheritance_vec,
        None => String::from("None")
    };

    PythonClass::new(class_name.to_string(), class_methods, class_inheritance, docstring)
}
