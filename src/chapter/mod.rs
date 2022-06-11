pub mod node;
pub mod types;
pub mod rules;
pub mod state;
pub mod processor;

const DOCSTRING_START: &str = r#"\s{4}"{3}\w*|\s{4}"{3}"#;
const DEF_MATCH: &str = r"def\s\w+";
const CLASS_MATCH: &str = r"class\s\w+";
const ALL_MATCH: &str = r"__all__\s=\s(\[|\()";
const DOCSTRING_END: &str = r#"('{3}|"{3})$"#;

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::chapter::{node::ContextNode, types::{ContextType, Indent}, processor::ContextProcessor};

    #[test]
    fn test_context_node_set_parent() {
        let parent_node = ContextNode::new(
            "TestClass".to_string(),
            ContextType::CLASS,
            0,
            true,
        );
        let child_new = ContextNode::new(
            "__init__".to_string(),
            ContextType::METHOD,
            0,
            true,
        );

        parent_node.borrow_mut().add_node(Rc::clone(&child_new));
        child_new.borrow_mut().set_parent(Rc::clone(&parent_node));

        assert_eq!(parent_node.borrow().children.len(), 1);
        assert_eq!(child_new.borrow().parent, Some(parent_node));
    }

    #[test]
    fn test_set_location() {
        let parent_node = ContextNode::new(
            "TestClass".to_string(),
            ContextType::CLASS,
            1,
            true,
        );

        assert_eq!(parent_node.borrow().location(), None);
        parent_node.borrow_mut().set_location(10);
        assert_eq!(parent_node.borrow().location(), Some((1, 10)));
    }

    #[test]
    fn test_append_value() {
        let parent_node = ContextNode::new(
            "TestClass".to_string(),
            ContextType::CLASS,
            1,
            true,
        );

        assert_eq!(parent_node.borrow().value, None);
        parent_node
            .borrow_mut()
            .append_value(&"Some test value".to_string());
        assert_eq!(
            parent_node.borrow().value,
            Some("Some test value".to_string())
        );
    }

    #[test]
    fn test_check_context_entry() {
        let blank_processor = ContextProcessor::load(Vec::new());
        let class_context = "class TestClass(TestInterface):  ".to_string();
        let method_context = "    def hello(args: int, test: str, **kwargs) -> None:".to_string();

        let class_result = blank_processor.check_context_entry(&class_context);
        let method_result = blank_processor.check_context_entry(&method_context);

        assert_eq!(class_result, Some(ContextType::CLASS));
        assert_eq!(method_result, Some(ContextType::METHOD));
    }

    #[test]
    fn test_start_context_from_root() {
        let mut blank_processor = ContextProcessor::load(Vec::new());
        let class_context = "class TestClass(TestInterface):  ".to_string();

        blank_processor.start_context(ContextType::CLASS, &class_context);

        assert_eq!(
            blank_processor.context_state.context_type,
            ContextType::CLASS
        );
        assert_eq!(
            blank_processor.context_state.context_node.borrow().name,
            "class TestClass"
        );
    }

    #[test]
    fn test_get_context_name() {
        let mut blank_processor = ContextProcessor::load(Vec::new());
        let mut class_context = "class TestClass(TestInterface):  ".to_string();

        blank_processor.context_state.context_type = ContextType::CLASS;
        let result = blank_processor.get_context_name(&class_context);
        assert_eq!(result, "class TestClass".to_string());

        class_context = "   async def test_method(self, args, kwargs) -> None:".to_string();
        blank_processor.context_state.context_type = ContextType::METHOD;
        let result = blank_processor.get_context_name(&class_context);
        assert_eq!(result, "def test_method".to_string());
    }

    #[test]
    fn test_check_context_exit_docstring_and_root() {
        let mut blank_processor = ContextProcessor::load(Vec::new());
        let current_line = " \"\"\"Docstring one liner\"\"\"".to_string();

        let result = blank_processor.check_context_exit(&current_line);
        assert_eq!(result, false);

        blank_processor.context_state.context_type = ContextType::DOCSTRING;
        let result = blank_processor.check_context_exit(&current_line);
        assert_eq!(result, true);

        let current_line = "   \"\"\"".to_string();
        let result = blank_processor.check_context_exit(&current_line);
        assert_eq!(result, true);

        let current_line = "   some text at the end\"\"\"".to_string();
        let result = blank_processor.check_context_exit(&current_line);
        assert_eq!(result, true);

        let current_line = "   \'\'\'".to_string();
        let result = blank_processor.check_context_exit(&current_line);
        assert_eq!(result, true);
    }

    #[test]
    fn test_check_context_exit_class() {
        let text_code = "
class TestClass:

    def __init__():
        pass


"
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        let mut blank_processor = ContextProcessor::load(text_code.clone());
        let current_line = text_code[5].clone();
        blank_processor.line_counter = 5;
        blank_processor.context_state.context_type = ContextType::CLASS;

        let result = blank_processor.check_context_exit(&current_line);
        assert_eq!(result, true);

        let current_line = text_code[4].clone();
        blank_processor.line_counter = 4;
        blank_processor.context_state.context_type = ContextType::CLASS;

        let result = blank_processor.check_context_exit(&current_line);
        assert_eq!(result, false);
    }

    #[test]
    fn test_check_context_exit_method() {
        let text_code = "
def __init__():
    pass

def hello() -> str:
    return hello world

"
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        let mut blank_processor = ContextProcessor::load(text_code.clone());
        let current_line = text_code[3].clone();
        blank_processor.line_counter = 3;
        blank_processor.context_state.context_type = ContextType::METHOD;

        let result = blank_processor.check_context_exit(&current_line);
        assert_eq!(result, true);
    }

    #[test]
    fn test_parse_module() {
        let text_code = "
class TestClass:

    def __init__(self):
        \"\"\" Niste docstring
        multiline
        \"\"\"
        pass

        def inner_method():
            pass


class NewClass:

    def __init__():
        pass

def outscope_method():
    pass

"
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

        let mut processor = ContextProcessor::load(text_code);

        let module = processor.parse_module();

        assert_eq!(module.borrow().children.len(), 3);
    }

    #[test]
    fn test_indent_struct() {
        let mut new_indent = Indent::new();
        new_indent.increase();
        assert_eq!(new_indent.value().len(), 4);

        new_indent.increase();
        assert_eq!(new_indent.value().len(), 8);

        new_indent.decrease();
        assert_eq!(new_indent.value().len(), 4);
    }
}
