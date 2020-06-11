extern crate clap;

use mditty::utils::*;

use std::env;
use std::path::Path;

use clap::{App, Arg};

fn main() {
    let current_dir = env::current_dir().unwrap_or_else(|why| {
        panic!(
            "Could not determine the current working directory: {}. There may not 
            be sufficient permissions to access the current directory.", why
        );
    });

    let matches = App::new("mditty")
        .version("0.1")
        .author("William Gasper <wkg@williamgasper.com>")
        .arg(
            Arg::with_name("INPUT")
                .short("i")
                .long("input")
                .default_value(current_dir.to_str().unwrap())
                .long_help(
                    "Input path, can be a file or directory. If not provided, the 
                      current working directory will be used. Directories will
                      be searched recursively and any pertinent files found will 
                      be converted to markdown.",
                ),
        )
        .get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let mut input_path = Path::new(input).to_path_buf();

    markdownify(input_path);
    //println!("{}", input_path.to_str().unwrap());
}
