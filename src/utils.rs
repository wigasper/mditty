use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, LineWriter};
use std::path::PathBuf;

// TODO: add logic to ignore directories, should be taken as an argument
pub fn find_files(input_path: &PathBuf, extension_map: &HashMap<String, String>, recurse: bool) {
    let mut dirs_to_search: Vec<PathBuf> = vec![input_path.to_path_buf()];

    while !dirs_to_search.is_empty() {
        let current_dir = dirs_to_search.pop().unwrap();

        for entry in current_dir.read_dir().expect("read_dir call failure") {
            if let Ok(entry) = entry {
                let entry_path = entry.path();

                if entry_path.is_file() {
                    let putative_md_path = get_output_path(&entry_path);
                    if !putative_md_path.exists() {
                        let _ = file_to_markdown(&entry_path, extension_map);
                    }
                } else if entry_path.is_dir() & recurse {
                    dirs_to_search.push(entry_path);
                }
            }
        }
    }
}

pub fn get_output_path(input_file_path: &PathBuf) -> PathBuf {
    let mut out_path = input_file_path.to_owned();
    out_path.set_extension("md");
    out_path
}

pub fn get_file_extension(file_path: &PathBuf) -> &str {
    let extension = file_path.extension().unwrap_or(OsStr::new(""));

    extension.to_str().unwrap()
}

pub fn file_to_markdown(file_path: &PathBuf, extension_map: &HashMap<String, String>) -> PathBuf {
    let file_extension = get_file_extension(file_path);

    let output_path = PathBuf::new();

    if extension_map.contains_key(file_extension) {
        // get output path
        let output_path = get_output_path(file_path);
        
        if !output_path.exists() {
            // create LineWriter for output
            let out_file = File::create(&output_path).unwrap_or_else(|why| {
                panic!("Could not create outfile: {}", why);
            });
            let mut out_file = LineWriter::new(out_file);

            // write header
            let header = format!("```{}\n", extension_map.get(file_extension).unwrap());
            out_file.write_all(header.as_bytes()).unwrap_or_else(|why| {
                panic!("Error writing header: {}", why);
            });

            // read in
            let file = File::open(file_path).unwrap_or_else(|why| {
                panic!("Could not open {}: {}", file_path.to_str().unwrap(), why);
            });

            let buf_reader = BufReader::new(file);

            for line in buf_reader.lines() {
                out_file
                    .write_all(line.unwrap().as_bytes())
                    .unwrap_or_else(|why| {
                        panic!("Could not write line from bufreader: {}", why);
                    });
                out_file.write_all(b"\n").unwrap_or_else(|why| {
                    panic!("Could not write newline after line: {}", why);
                });
            }

            // write footer
            out_file.write_all(b"```").unwrap_or_else(|why| {
                panic!("Error writing footer: {}", why);
            });

            out_file.flush().unwrap();
        }
    }

    output_path
}

pub fn sanity_check(input_path: &PathBuf) -> bool {
    let mut sane = false;

    if input_path == &PathBuf::from("/") {
        println!("Warning: mditty should not be run on root directory");
        println!("If you actually want to do this open an issue and 
                 I will add some logic to allow for this");
    } else {
        sane = true;
    }
    
    sane
}

pub fn markdownify(mut input_path: PathBuf, recurse: bool) {
    input_path = input_path.canonicalize().unwrap();

    let extension_map = get_ext_map();
    
    let sane = sanity_check(&input_path);

    if input_path.is_file() & sane {
        file_to_markdown(&input_path, &extension_map);
    } else if input_path.is_dir() & sane {
        find_files(&input_path, &extension_map, recurse);
    } else {
        // something is wrong??? is this condition possible?
    }
}

pub fn get_ext_map() -> HashMap<String, String> {
    let mut map_out = HashMap::new();
    let extensions = vec![
        ("awk", "awk"),
        ("c", "c"),
        ("cpp", "cpp"),
        ("cs", "c#"),
        ("css", "css"),
        ("csv", "csv"),
        ("go", "go"),
        ("java", "java"),
        ("js", "javascript"),
        ("php", "php"),
        ("pl", "perl"),
        ("py", "python"),
        ("sh", "bash"),
        ("slurm", "bash"),
        ("sql", "sql"),
        ("r", "r"),
        ("R", "r"),
        ("rs", "rust"),
        ("tex", "latex"),
    ];

    for ext in extensions.iter() {
        map_out.insert(ext.0.to_owned(), ext.1.to_owned());
    }

    map_out
}
