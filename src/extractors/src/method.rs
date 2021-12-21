use crate::{FUNCTION_KEYWORD, ENDEF_KEYWORD};
use crate::parameter::extract_parameters;
use crate::pyobjects::Method;

use helpers::regex_split;


/// Extract a python method name from its method header
///
/// # Arguments
///
/// * `method_header`: Method header code line
///
/// # Example
/// ```python
/// def method_name(self, arg1: int, arg2: str) -> None:
/// ```
/// Extracted name here is `method_name`
pub fn extract_method_name(method_header: &String) -> Result<String, &str> {
    let split_header = regex_split(r"\W", true, method_header);
    if split_header[0].trim() != FUNCTION_KEYWORD.trim() {
        return Err("This is not a method header")
    }
    Ok(split_header[1].to_string())
}


/// Extract method output after the pointing arrow
///
/// # Arguments
///
/// * `header` - Python method header
///
/// # Output
///
/// * `Err` - if the header had no type and at split nothing happened
/// * `Ok` - returns header type
pub fn extract_method_output(header: &String) -> Result<String, &str> {
    let header_split = regex_split(r"( -> )", true, header);
    match header_split.len() {
        1 => Err("Output type not found"),
        _ => Ok(header_split[1].trim().replace(":", ""))
    }
}

/// Extract methods found in a Python class
///
/// # Arguments
///
/// * `class_code` - The code for the Python class extracted from the
/// .py file
pub fn extract_methods(class_code: &Vec<String>) -> Vec<Method> {

    // Initialize temp method and start for retrieving method headers
    // that span on multiple lines
    let mut methods: Vec<Method> = Vec::new();
    let mut temp_method = String::new();
    let mut start = false;

    for line in class_code.iter() {
        if !start && line.contains(FUNCTION_KEYWORD) { start = true; }
        if start {
            temp_method.push_str(format!(" {}", line.trim()).as_str());
            let last_char = match temp_method.chars().nth(temp_method.len() - 1) {
                Some(chr) => chr,
                None => continue
            };
            if last_char == ENDEF_KEYWORD {
                let method_name = match extract_method_name(&temp_method) {
                    Ok(name) => name,
                    Err(err) => {
                        println!("Error while extracting method name: {}", err);
                        String::from("ENL")
                    }
                };
                let parameters = extract_parameters(&temp_method);
                let method_output = match extract_method_output(&temp_method) {
                    Ok(output) => output,
                    Err(_) => String::from("None")
                };
                methods.push(Method::new(method_name, parameters, method_output));
                temp_method = String::new();
                start = false;
            }
        }
    }
    methods
}