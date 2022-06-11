/*
JonesCLI

Author: Vlad Nedelcu
Date: Jul 2021
License: MIT

Copyright 2021 Vlad Nedelcu
*/

pub mod utils;
pub mod objects;
pub mod display;
pub mod markers;
pub mod docstrings;

use std::fs;
use std::path::PathBuf;

use crate::chapter::processor::ContextProcessor;
use crate::chapter::types::ContextType;

const CLASS_TEMPLATE_INHERITANCE: &str = "class {template}(";
const CLASS_TEMPLATE: &str = "class {template}:";
const TEMPLATE_KEYWORD: &str = "{template}";
const PYTHON_EXTENSION: &str = "py";
const DOCSTRING: &str = "\"\"\"";
const NEWLINE: &str = "\n";
const INIT_DEF: &str = "def __init__";


type ClassMatch = (String, String);


/// Extracts the searched python class from the code
///
/// # Arguments
///
/// * `code_lines`: The full split into lines code file
///
/// * `class_name`: The searched class name
///
fn extract_python_class(code_lines: Vec<&str>, class_name: &str) -> objects::PythonClass {
    let class_name_inheritance = CLASS_TEMPLATE_INHERITANCE.clone()
        .replace(TEMPLATE_KEYWORD, class_name);
    let class_name_simple = CLASS_TEMPLATE.clone()
        .replace(TEMPLATE_KEYWORD, class_name);

    let mut start_cutting: bool = false;
    let mut class_code_block: Vec<String> = Vec::new();
    let mut class_header: String = String::from("");

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

    let class_inheritance = match utils::extract_class_inheritance(&class_header) {
        Some(inheritance_vec) => inheritance_vec,
        None => Vec::new()
    };

    let docstring = match docstrings::extract_docstring(&class_code_block) {
        Some(inheritance_vec) => inheritance_vec,
        None => String::from("None")
    };

    objects::PythonClass::new(class_code_block, class_name.to_string(), class_inheritance, docstring)
}

/// Check if a file contains the searched class by reading the file.
///
/// # Arguments
/// * `class_name`: The class name of the Python class
/// * `file_path`: The file path to be read
///
/// # Errors
/// It panics if the file is cannot be read properly
fn check_file_contains_class(class_name: &str, file_path: &str) -> bool {
    let class_name_inheritance = CLASS_TEMPLATE_INHERITANCE.clone()
        .replace(TEMPLATE_KEYWORD, class_name);
    let class_name = CLASS_TEMPLATE.clone()
        .replace(TEMPLATE_KEYWORD, class_name);

    match fs::read_to_string(file_path) {
        Ok(file_content) => {
            let first_check = file_content.contains(&class_name_inheritance);
            let second_check = file_content.contains(&class_name);
            return first_check || second_check
        },
        Err(_) => {
            return false
        }
    };
}

/// Searches recurssively through a project for a Python class and extracts that
/// class into an PythonClass struct.
pub fn project_traversal(dir_path: &PathBuf, class_name: &String) -> Option<objects::PythonClass> {
    let current_dir = match fs::read_dir(dir_path) {
        Ok(dir) => dir,
        Err(err) => {
            println!("Error occured while reading dir: {}", err);
            return None
        }
    };

    for file in current_dir {
        let file_path = file.unwrap().path();
        let file_path_name = file_path.to_str().unwrap();
        if file_path.is_dir() {
            match project_traversal(&file_path, class_name) {
                Some(value) => {
                   return Some(value)
                },
                None => continue
            };
        } else {
            match file_path.extension() {
                Some(extension) => {
                    if extension != PYTHON_EXTENSION {
                        continue
                    }
                },
                None => continue
            }
            if check_file_contains_class(class_name, &file_path_name){
                let file_content = match fs::read_to_string(file_path) {
                    Ok(content) => content,
                    Err(_) => {
                        println!("Now skipping");
                        continue
                    }
                };
                let lines: Vec<&str> = file_content.split("\n").collect();

                return Some(extract_python_class(lines, class_name))
            }
        }
    }
    return None
}

/// Project traversal recursive and searches for a keyword based on itself or on context (Phase 2)
pub fn smart_search(dir_path: &PathBuf, class_name: &String)  -> Option<Vec<ClassMatch>>{
    let mut found_matched_classes: Vec<ClassMatch> = Vec::new();

    let current_dir = match fs::read_dir(dir_path) {
        Ok(dir) => dir,
        Err(err) => {
            println!("Error occured while reading dir: {}", err);
            return None
        }
    };

    for file in current_dir {
        let file_path = file.unwrap().path();
        let file_path_name = file_path.to_str().unwrap();
        if file_path.is_dir() {
            match smart_search(&file_path, class_name) {
                Some(matches) => found_matched_classes.extend(matches),
                None => continue
            };
        } else {
            match file_path.extension() {
                Some(extension) => {
                    if extension != PYTHON_EXTENSION {
                        continue
                    }
                },
                None => continue
            };
            let file_content = match fs::read_to_string(&file_path) {
                Ok(content) => content,
                Err(_) => continue
            };
            if !file_content.contains(class_name) {
                continue
            }
            let lines = file_content
                            .split("\n")
                            .map(|line| line.to_string())
                            .collect::<Vec<String>>();

            let mut processor = ContextProcessor::load(lines);
            let module = processor.parse_module();

            for child in &module.borrow().children {
                let ch = child.borrow();
                if ch.context_type == ContextType::CLASS && ch.name.contains(class_name) {
                    found_matched_classes.push((ch.name.to_string(), file_path_name.to_string()))
                }
            }

        }
    }

    if found_matched_classes.len() > 0 {
        Some(found_matched_classes)
    } else {
        None
    }
}


#[cfg(test)]
mod tests {
    use super::objects::PythonClass;
    use super::extract_python_class;
    use super::project_traversal;
    use super::check_file_contains_class;
    use std::fs;
    use std::path::PathBuf;

    static PYTHON_CODE: &str = "
    class God:
        \"\"\"DocString\"\"\"
        def __init__(self, name: int):
            self.name == name

        def hi(self) -> None:
            print(f'My name is {self.name}')

    ";
    static RANDOM_CODE: &str = "
    class TestClass:

        def __init__(self, age: gig):
            self.age == age

        def hi(self):
            print(f'My name is {self.age}')
    ";

    #[test]
    fn test_extract_python_class(){

        let test_codebase = vec![
            "class God:".to_string(),
            "    \"\"\"DocString\"\"\"".to_string(),
            "".to_string(),
            "    def __init__(self, name: int):".to_string(),
            "        self.name = name".to_string(),
            "".to_string(),
            "    def hi(self):".to_string(),
            "        print(f'My name is {self.name}')".to_string(),
            "".to_string()
        ];
        let lines: Vec<&str> = PYTHON_CODE.split("\n").collect();

        let expected_class = PythonClass::new(test_codebase, String::from("God"), Vec::new(), String::from("DocString"));

        assert_eq!(extract_python_class(lines, "God"), expected_class);

    }

    #[test]
    fn test_check_file_contains_class() {
        let path = "./test.py";
        fs::write(path, PYTHON_CODE).unwrap();

        assert_eq!(true, check_file_contains_class("God", path));
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_check_file_contains_class_err() {
        assert_eq!(check_file_contains_class("God", "./iogh.py"), false);
    }

    #[test]
    fn test_project_traversal() {
        let path = "./testing/test.py";
        let mut pathbuf = PathBuf::new();
        pathbuf.push("./testing");
        fs::create_dir("./testing").expect("Could not write dire");
        fs::write(path, PYTHON_CODE).unwrap();


        let test_codebase = vec![
            "class God:".to_string(),
            "    \"\"\"DocString\"\"\"".to_string(),
            "".to_string(),
            "    def __init__(self, name: int):".to_string(),
            "        self.name = name".to_string(),
            "".to_string(),
            "    def hi(self):".to_string(),
            "        print(f'My name is {self.name}')".to_string(),
            "".to_string()
        ];

        let expected_class = PythonClass::new(test_codebase, String::from("God"), Vec::new(), String::from("DocString"));
        assert_eq!(expected_class, project_traversal(&pathbuf, &"God".to_string()).unwrap());

        fs::remove_dir_all("./testing").expect("Could not delete dir");
    }

    #[test]
    fn test_process_only_py_files() {

        // Paths
        let test_dir = String::from("./testing_none");
        let python_file = String::from("./testing_none/test.py");
        let random_file = String::from("./testing_none/test.rs");
        let mut pathbuf = PathBuf::new();

        // Create dir and files
        fs::create_dir(test_dir).expect("Could not write dir");
        fs::write(python_file, PYTHON_CODE).unwrap();
        fs::write(random_file, RANDOM_CODE).unwrap();
        pathbuf.push("./testing_none");

        // Assert
        assert_eq!( project_traversal(&pathbuf, &"TestCode".to_string()), None);

        // Destroy the test dir
        fs::remove_dir_all("./testing_none").expect("Could not delete dir");
    }
}