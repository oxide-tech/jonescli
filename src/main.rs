/*
JonesCLI

Author: Vlad Nedelcu
Date: Jul 2021
License: MIT

Copyright 2021 Vlad Nedelcu
*/

mod joneslib;
mod commands;
pub mod chapter;

use structopt::StructOpt;
use joneslib::display;

fn main() {
    let comms = commands::CLI::from_args();
    if comms.grep {
        // Search for a keyword in class name
        match joneslib::smart_search(&comms.dir_path, &comms.class_name) {
            Some(matches) => display::class_matches(matches),
            None => display::not_found_message()
        }
    } else {
        // Generate python class
        match joneslib::project_traversal(&comms.dir_path, &comms.class_name) {
            Some(class) => joneslib::display::output_class(&class),
            None => display::not_found_message()
        }
    }
}

