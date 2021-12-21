use std::fs;
use std::path::PathBuf;

use extractors::PythonClass;
use extractors::extract_python_class;

use crate::{PYTHON_EXTENSION, TEMPLATE_KEYWORD, CLASS_TEMPLATE_INHERITANCE, CLASS_TEMPLATE};

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
            first_check || second_check
        },
        Err(_) => {
            return false
        }
    }
}

/// Searches recursively through a project for a Python class and extracts that
/// class into an PythonClass struct.
pub fn project_traversal(dir_path: &PathBuf, class_name: &String) -> Option<PythonClass> {
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