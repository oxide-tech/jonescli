mod search;
mod traversal;

pub use search::smart_search;
pub use traversal::project_traversal;

static CLASS_KEYWORD: &str = "class ";
static PYTHON_EXTENSION: &str = "py";
static CLASS_TEMPLATE_INHERITANCE: &str = "class {template}(";
static CLASS_TEMPLATE: &str = "class {template}:";
static TEMPLATE_KEYWORD: &str = "{template}";


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
