use ansi_term::Colour;
use extractors::PythonClass;

type ClassMatch = (String, String);

pub fn output_class(python_class: &PythonClass) {
    println!("{}", python_class);

    for method in python_class.methods.iter() {
        println!("{}", method);
        for parameter in method.parameters.iter() {
            println!("{}", parameter);
        }
    }
}

pub fn not_found_message() -> () {
    println!(
        "{}: {}",
        Colour::Green.paint("Output"),
        Colour::Yellow.paint("Searched class was not found in project")
    )
}

pub fn class_matches(found_match_classes: Vec<ClassMatch>) -> () {
    println!("> [{}]", Colour::Cyan.paint("FOUND MATCHES"));
    for line in found_match_classes.iter() {
        println!(
            ":: {} -> {}",
            Colour::Yellow.paint(&line.0.replace("\r", "")),
            Colour::Purple.paint(&line.1)
        )
    }
}