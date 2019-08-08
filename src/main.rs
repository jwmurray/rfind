extern crate clap;
extern crate walkdir;

use clap::App;
use clap::Arg;

use walkdir::{DirEntry, WalkDir};


fn main() {
    App::new("rfind")
        .version("1.0")
        .about("Unix Find Replacement")
        .author("John M.")
        .arg(
            Arg::with_name("regex_pattern")
                .short("r")
                .long("regex")
                .value_name("regex_pattern")
                .help("regex pattern for the file systen entry name")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("path")
                .help("Sets the search path")
                .default_value(".")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::with_name("inner_pattern")
                .help("Specify pattern for file system entry.")
                .default_value(".*")
                .required(false)
                .index(2),
        )
        .get_matches();
}
