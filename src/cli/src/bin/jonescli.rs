/*
JonesCLI

Author: Vlad Nedelcu
Date: Jul 2021
License: MIT

Copyright 2021 Vlad Nedelcu
*/
use cli::CLI;
use cli::display;

use structopt::StructOpt;
use navigation;

fn main() {
    let comms: CLI = CLI::from_args();

    if comms.grep {
        // Search for python classes using the class keyword
        match navigation::smart_search(&comms.dir_path, &comms.class_name) {
            Some(matches) => display::class_matches(matches),
            None => display::not_found_message()
        }
    } else {
        match navigation::project_traversal(&comms.dir_path, &comms.class_name){
            Some(py_class) => display::output_class(&py_class),
            None => display::not_found_message()
        }
    }
}