
use crate::OPEN_PARENTHESES;
use crate::CLOSE_PARENTHESES;
use crate::COMMA;
use regex::Regex;

/// Fetch the arguments from a class method and mark the commas
/// that will be used for splitting further
///
/// # Arguments
///
/// * `header` - The method header
///
pub fn get_header_arguments(header: &String) -> Option<String> {
    let reg = Regex::new(r".*?\(|\).*").unwrap();
    let segments: Vec<&str> = reg.split(header)
        .filter(|&entry| !entry.is_empty())
        .collect();

    match segments.len() {
        0 => None,
        _ => Some(mark_commas_for_split(segments[0]))
    }
}

/// Mark the commas that are not dividing the arguments in a method
/// class
///
/// # Arguments
///
/// * `args` - Arguments segment from a method header
pub fn mark_commas_for_split(args: &str) -> String {
    let mut inside_brackets: bool = false;
    let mut marked_args = args.to_string();

    for (pos, ch) in args.chars().enumerate() {
        match ch {
            OPEN_PARENTHESES => inside_brackets = true,
            CLOSE_PARENTHESES => inside_brackets = false,
            COMMA => {
                if inside_brackets {
                    marked_args.remove(pos+1);
                }
            },
            _ => continue
        }
    }

    marked_args
}