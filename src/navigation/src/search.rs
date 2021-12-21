use crate::{PYTHON_EXTENSION, CLASS_KEYWORD};
use std::path::PathBuf;
use std::fs;


type ClassMatch = (String, String);

/// Search & find all Python classes that contain the keyword or are relevant to the context
///
/// # Arguments
///
/// * `lines` - The python file code lines previously read
/// * `keyword` - The class name given for the search
///
/// # Output
///
/// * `Option<Vec<String, String>>` - containing all the found relevant classes
pub fn grep_class<'a>(lines: Vec<&str>, keyword: &String, file_name: &str) -> Option<Vec<(String, String)>> {
    let mut found_match_classes: Vec<(String, String)> = Vec::new();
    for line in lines.iter() {
        if line.trim().starts_with(CLASS_KEYWORD) && line.contains(keyword) {
            found_match_classes.push(
                (line.to_string(), file_name.to_string())
            );
        }
    }
    match found_match_classes.len() {
        0 => None,
        _ => Some(found_match_classes)
    }
}

/// Project traversal recursive and searches for a keyword based on itself or on context (Phase 2)
pub fn smart_search(dir_path: &PathBuf, class_name: &String)  -> Option<Vec<ClassMatch>>{
    let mut found_matched_classes: Vec<ClassMatch> = Vec::new();

    let current_dir = match fs::read_dir(dir_path) {
        Ok(dir) => dir,
        Err(err) => {
            println!("Error occurred while reading dir: {}", err);
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
            let lines: Vec<&str> = file_content.split("\n").collect();
            match grep_class(lines, &class_name, file_path_name) {
                Some(matches) => found_matched_classes.extend(matches),
                None => continue
            }
        }
    }

    if found_matched_classes.len() > 0 {
        Some(found_matched_classes)
    } else {
        None
    }
}