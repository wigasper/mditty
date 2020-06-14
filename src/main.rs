extern crate clap;

use mditty::utils::*;

use std::env;
use std::path::Path;

use clap::{App, Arg};

fn main() {
    let current_dir = env::current_dir().unwrap_or_else(|why| {
        panic!("Could not determine the current working directory: {}", why);
    });

    let matches = App::new("mditty")
        .version("0.1")
        .author("William Gasper <wkg@williamgasper.com>")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .default_value(current_dir.to_str().unwrap())
                .long_help(
                    "Input path, can be a file or directory. If not 
provided, the current working directory will be 
used. Directories will be searched recursively and 
any pertinent files found will be converted to markdown.",
                ),
        )
        .arg(
            Arg::with_name("recursive")
                .short("r")
                .long("recurse")
                .long_help(
                    "If present and if a directory is provided as 
input, then specifying this argument will result in 
the directory being recursively searched and all files 
that can be converted to markdown will be.",
                ),
        )
        .arg(
            Arg::with_name("extensions")
                .short("e")
                .long("extensions")
                .help("Show currently supported extensions"),
        )
        .get_matches();

    if matches.is_present("extensions") {
        let temp_map = mditty::utils::get_map();

        for key in temp_map.keys() {
            println!("{}", key);
        }
    }

    let mut recurse = false;

    if matches.is_present("recursive") {
        recurse = true;
    }

    if !matches.is_present("extensions") {
        let input = matches.value_of("input").unwrap();
        let input_path = Path::new(input).to_path_buf();

        markdownify(input_path, recurse);
    }
}
