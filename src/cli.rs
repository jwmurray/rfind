use clap::App;
use clap::Arg;

use ::regex;
use regex::Regex;

#[derive(Debug)]

pub struct Options  {
    path: String,
    pattern: Regex,
    // path_slice: &'a str,
    // pattern_slice: &'a str,
}

pub fn get_options() -> Options {
    // let mut opt
    let matches = App::new("rfind")
        .version("1.0")
        .about("Unix Find Replacement")
        .author("John M.")
        .arg(
            Arg::with_name("regex_pattern")
                .short("r")
                .long("regex")
                .default_value(".*")
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
    let path = matches.value_of("path").unwrap_or("default.path");
    let pattern_string
     = matches
        .value_of("regex_pattern")
        .unwrap_or("default.regex_pattern");

    let mut pattern = Option<Regex> = None;
    pattern = match regex::Regex::new(pattern ) {
        Ok(regex) => Some(regex),
        Err(error) => return Err(ParseError(format!("Couldn't compile regex pattern. {}", error)))
    }

    println!("Path: {}, pattern: {}", path, pattern);
    Options {
        path: String::from(path),
        pattern: String::from(pattern),
    }
}
