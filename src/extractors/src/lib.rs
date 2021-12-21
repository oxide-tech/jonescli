mod pyobjects;
mod parameter;
mod method;
mod class;
mod docstring;

pub use crate::{
    parameter::extract_parameters,
    method::extract_methods,
    class::extract_python_class,
    docstring::extract_docstring
};

pub use pyobjects::{Parameter, Method, PythonClass};


static FUNCTION_KEYWORD: &str = " def ";
static DEFAULT_TYPE: &str = "None";
static ENDEF_KEYWORD: char = ':';
static CLASS_TEMPLATE_INHERITANCE: &str = "class {template}(";
static CLASS_TEMPLATE: &str = "class {template}:";
static TEMPLATE_KEYWORD: &str = "{template}";
static DOCSTRING: &str = "\"\"\"";
static NEWLINE: &str = "\n";

#[cfg(test)]
mod extractors_tests {

    use crate::{
        docstring::{format_line, get_docstring, extract_docstring},
        method::{
            extract_method_name,
            extract_methods,
            extract_method_output
        },
        parameter::extract_parameters,
        pyobjects::{
            Parameter,
            Method
        },
        NEWLINE,
        class::{
            extract_class_inheritance
        }
    };

    #[test]
    fn test_format_line(){
        let test_string = "\"\"\"Test docstring";
        let expected = "Test docstring";

        assert_eq!(format_line(test_string), expected);
    }

    #[test]
    fn test_get_docstring(){
        let test_code_block: Vec<String> = vec![
            "class God:".to_string(),
            "    \"\"\"DocString\"\"\"".to_string(),
            "".to_string(),
            "    def __init__(self, name: int):".to_string(),
            "        self.name = name".to_string(),
            "".to_string()
        ];
        let expected = vec!["DocString".to_string()];

        assert_eq!(get_docstring(&test_code_block), Some(expected));
    }

    #[test]
    fn test_get_docstring_none(){
        let test_code_block: Vec<String> = vec![
            "class God:".to_string(),
            "    pass".to_string(),
            "".to_string()
        ];
        assert_eq!(get_docstring(&test_code_block), None);
    }

    #[test]
    fn test_extract_docstring_some(){
        let test_code_block: Vec<String> = vec![
            "class God:".to_string(),
            "    \"\"\"".to_string(),
            "     DocString".to_string(),
            "     Some more test".to_string(),
            "    \"\"\"".to_string(),
            "".to_string(),
            "    def __init__(self, name: int):".to_string(),
            "        self.name = name".to_string(),
            "".to_string()
        ];
        let expected = vec![
            "".to_string(),
            "DocString".to_string(),
            "Some more test".to_string(),
            "".to_string()
        ];

        assert_eq!(extract_docstring(&test_code_block), Some(expected.join(NEWLINE)))
    }

    #[test]
    fn test_extract_method_name(){
        let test_string = String::from("def this_name(self, param2: int) -> None:");
        let expected = String::from("this_name");

        assert_eq!(extract_method_name(&test_string).unwrap(), expected);
    }

    #[test]
    fn test_extract_method_name_negative(){
        let test_string = String::from("import definition as positive");
        assert!(extract_method_name(&test_string).is_err());
    }

    #[test]
    fn test_extract_parameters_positive(){
        let test_string = String::from("def this_name(param1: str, param2: int) -> None:");
        let expected_parameters = vec![
            Parameter::new(String::from("param1"), String::from("str")),
            Parameter::new(String::from("param2"), String::from("int")),
        ];

        assert_eq!(extract_parameters(&test_string), expected_parameters);
    }

    #[test]
    fn test_extract_parameters_one_parameter(){
        let test_string = String::from("def this_name(self) -> None:");
        let expected_parameters = vec![
            Parameter::new(String::from("self"), String::from("None")),
        ];

        assert_eq!(extract_parameters(&test_string), expected_parameters);
    }

    #[test]
    fn test_extract_parameters_no_parameter(){
        let test_string = String::from("def this_name() -> None:");
        let expected_parameters = Vec::new();

        assert_eq!(extract_parameters(&test_string), expected_parameters);
    }

    #[test]
    fn test_extract_methods_positive(){
        let test_codebase = vec![
            "class Test:".to_string(),
            "".to_string(),
            "    def __init__(self, name):".to_string(),
            "        self.name = name".to_string(),
            "".to_string(),
            "    def say_hi(self):".to_string(),
            "        self.name = name".to_string(),
            "".to_string()
        ];

        let expected_methods = vec![
            Method::new(
                String::from("__init__"),
                vec![
                    Parameter::new(String::from("self"), String::from("None")),
                    Parameter::new(String::from("name"), String::from("None")),
                ],
                String::from("None")

            ),
            Method::new(
                String::from("say_hi"),
                vec![
                    Parameter::new(String::from("self"), String::from("None")),
                ],
                String::from("None")
            ),
        ];

        assert_eq!(extract_methods(&test_codebase), expected_methods);
    }

    #[test]
    fn test_extract_methods_multiple_lines(){
        let test_codebase = vec![
            "class Test:".to_string(),
            "".to_string(),
            "    def __init__(self, name: int,".to_string(),
            "                 param1: str,".to_string(),
            "                 param2: int) -> str:".to_string(),
            "        self.name = name".to_string(),
            "".to_string(),
            "    def say_hi(self):".to_string(),
            "        self.name = name".to_string(),
            "".to_string()
        ];

        let expected_methods = vec![
            Method::new(
                String::from("__init__"),
                vec![
                    Parameter::new(String::from("self"), String::from("None")),
                    Parameter::new(String::from("name"), String::from("int")),
                    Parameter::new(String::from("param1"), String::from("str")),
                    Parameter::new(String::from("param2"), String::from("int"))
                ],
                String::from("str")
            ),
            Method::new(
                String::from("say_hi"),
                vec![
                    Parameter::new(String::from("self"), String::from("None")),
                ],
                String::from("None")
            ),
        ];

        assert_eq!(extract_methods(&test_codebase), expected_methods);
    }

    #[test]
    fn test_extract_method_output() {
        let test_string = String::from("def this_name(self, param2: int) -> List[int]:");
        let expected = String::from("List[int]");

        assert_eq!(extract_method_output(&test_string).unwrap(), expected);
    }

    #[test]
    fn test_extract_class_inheritance() {
        let test_header = String::from("class Human(Being, Earthling):");
        let expected = vec![String::from("Being"), String::from("Earthling")];

        assert_eq!(extract_class_inheritance(&test_header), Some(expected));
    }

    #[test]
    fn test_extract_no_inheritance(){
        let test_header = String::from("class Human:");

        assert_eq!(extract_class_inheritance(&test_header), None);
    }
}