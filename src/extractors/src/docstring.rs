use crate::{DOCSTRING, NEWLINE};

pub fn extract_docstring(code_block: &Vec<String>) -> Option<String> {
    let docstring_vec = match get_docstring(&code_block) {
        Some(docstring) => docstring,
        None => return None
    };

    return Some(docstring_vec.join(NEWLINE))
}

pub(crate) fn get_docstring(code_block: &Vec<String>) -> Option<Vec<String>> {
    let mut start_docstring: bool = false;
    let mut docstring_vec: Vec<String> = Vec::new();

    for line in code_block.iter() {
        if !start_docstring && line.contains(DOCSTRING) {
            start_docstring = true;
            docstring_vec.push(format_line(line));
            match line.matches(DOCSTRING).count() {
                2 => break,
                _ => continue
            }
        }

        if start_docstring {
            docstring_vec.push(format_line(line));
            match line.contains(DOCSTRING) {
                true => break,
                false => continue
            }
        }
    }

    match docstring_vec.len() {
        0 => None,
        _ => Some(docstring_vec)
    }
}

pub(crate) fn format_line(line: &str) -> String {
    return line.trim().replace(DOCSTRING, "")
}