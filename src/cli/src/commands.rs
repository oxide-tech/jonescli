use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct CLI {

    // Flag to search all classes with that value
    #[structopt(short="g", long="grep",
        help="Used to retrieve all classes with that pattern")]
    pub grep: bool,

    // Class name to be fetched
    #[structopt(help="Name of the Python class")]
    pub class_name: String,

    // Search directory
    #[structopt(parse(from_os_str), default_value=".",
        help="Search directory")]
    pub dir_path: PathBuf
}