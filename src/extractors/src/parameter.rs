use helpers::markers::get_header_arguments;
use helpers::regex_split;

use crate::pyobjects::Parameter;
use crate::DEFAULT_TYPE;

/// Extract method parameters with their static type
///
/// # Arguments
///
/// * `header`: - The header line from a Python method
pub fn extract_parameters(header: &String) -> Vec<Parameter> {

    // Split to get all the parameter
    let parameter_segment: String = match get_header_arguments(header) {
        Some(params) => params,
        None => return Vec::new()
    };
    let parameters_values = regex_split(r",\s", true, &parameter_segment);
    let mut parameters: Vec<Parameter> = Vec::new();
    for param in parameters_values.iter(){
        let param_string = param.to_string();
        let param_values: Vec<&str> = regex_split(r":\s", false, &param_string);
        match param_values.len() {
            1 => {
                let par_name = param_values[0].trim().to_string();
                let par_type = DEFAULT_TYPE.to_string();
                parameters.push(Parameter::new(par_name, par_type));
            },
            2 => {
                let par_name = param_values[0].trim().to_string();
                let par_type = param_values[1].trim().to_string();
                parameters.push(Parameter::new(par_name, par_type));
            },
            _ => println!("Found method in code with no parameters. {:?}", param_values)
        };
    }
    parameters
}