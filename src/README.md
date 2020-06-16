# mditty
mditty is a fun little utility that, when passed either a file or a directory as input, will change that file or any files in the directory (and any subdirectories, if the recursive flag is set) into markdown if the file or files end in a handful of common extensions.
It does this simply by adding backticks and an accepted language identifier as the first line and backticks as the last line.

## Why would you do this?
I made this to assist in an automated documentation generation process. Basically, a bunch of scripts go into folders and I want them all turned into markdown with syntax highlighting.

## Usage
```
USAGE:
    mditty [FLAGS] [OPTIONS]

FLAGS:
    -e, --extensions    Show currently supported extensions
    -h, --help          Prints help information
    -r, --recurse       If present and if a directory is provided as 
                        input, then specifying this argument will result in 
                        the directory being recursively searched and all files 
                        that can be converted to markdown will be.
    -V, --version       Prints version information

OPTIONS:
    -i, --input <input>    Input path, can be a file or directory. If not 
                           provided, the current working directory will be 
                           used. If the recursive flag is set, then directories 
                           will be searched recursively and any pertinent files 
                           found will be converted to markdown. 
			   [default: /your/current/directory]
```

## Installation
Requires [Rust](https://rust-lang.org/tools/install). Then:
```
$ git clone https://github.com/wigasper/mditty
$ cd mditty
$ cargo build --release
```
The binary, at `target/release/mditty`, can be used from there, or feel free to move it to some location in your PATH for easy usage.
