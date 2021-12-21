/*
JonesCLI

Author: Vlad Nedelcu
Date: Jul 2021
License: MIT

Copyright 2021 Vlad Nedelcu
*/
pub mod markers;

use regex::Regex;

const OPEN_PARENTHESES: char = '[';
const CLOSE_PARENTHESES: char = ']';
const COMMA: char = ',';

/// Simple regex split on a given code line
/// # Arguments
///
/// * `r`: Regex string line
/// * `trim`: boolean if you want to trim the values
/// * `value`: value to split
pub fn regex_split<'a>(r: &'a str, trim: bool, value: &'a String) -> Vec<&'a str> {
    let regex_separator = Regex::new(r).expect("Invalid regex function given");
    if trim {
        return regex_separator.split(value.trim()).collect();
    }
    return regex_separator.split(value).collect();
}


#[cfg(test)]
mod test_helpers {

    use crate::markers::mark_commas_for_split;
    use crate::markers::get_header_arguments;
    use crate::regex_split;

    #[test]
    fn test_regex_split_positive(){
        let test_string = String::from("test1:test2");
        let expected = vec!["test1", "test2"];

        let values: Vec<&str> = regex_split(r":", false, &test_string);

        assert_eq!(values, expected);
    }

    #[test]
    fn test_regex_split_trim(){
        let test_string = String::from("   test1:test2   ");
        let expected = vec!["test1", "test2"];

        let values: Vec<&str> = regex_split(r":", true, &test_string);

        assert_eq!(values, expected);
    }

    #[test]
    fn test_mark_commas_for_split() {
        let args = "param1: str, param2: Dict[str, int]";
        let expected = String::from("param1: str, param2: Dict[str,int]");

        assert_eq!(mark_commas_for_split(args), expected);
    }

    #[test]
    fn test_get_header_arguments(){
        let header = String::from("def test_method(param1: str, param2: Dict[str, int]) -> str");
        let expected = String::from("param1: str, param2: Dict[str,int]");

        assert_eq!(get_header_arguments(&header), Some(expected));
    }

    #[test]
    fn test_get_header_no_arguments(){
        let header = String::from("def test_method() -> str");
        assert_eq!(get_header_arguments(&header), None);
    }
}
