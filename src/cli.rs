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
        .get_matches();

    let path = matches.value_of("path").unwrap_or("default.path");
    let arg
     = matches.value_of("regex_pattern").unwrap_or("default.regex_pattern");

    let mut pattern: Option<Regex> = None;
    pattern = match regex::Regex::new(arg ) {
        Ok(regex) => Some(regex),
        Err(error) => return Err(ParseError(format!("Couldn't compile regex pattern. {}", error))),
    }

    Options("hello".to_string(), pattern)
}

