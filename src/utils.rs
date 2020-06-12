use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, LineWriter};
use std::path::{Path, PathBuf};

pub fn find_files(input_path: &PathBuf, extension_map: &HashMap<String, String>) {
    let mut dirs_to_search: Vec<PathBuf> = vec![input_path.to_path_buf()];

    while !dirs_to_search.is_empty() {
        let current_dir = dirs_to_search.pop().unwrap();

        for entry in current_dir.read_dir().expect("read_dir call failure") {
            if let Ok(entry) = entry {
                let entry_path = entry.path();

                if entry_path.is_file() {
                    file_to_markdown(&entry_path, extension_map);
                } else if entry_path.is_dir() {
                    dirs_to_search.push(entry_path);
                }
            }
        }
    }
}

pub fn file_to_markdown(file_path: &PathBuf, extension_map: &HashMap<String, String>) {
    // TODO: handle this better, probably should have its own function
    let file_extension = file_path.extension().unwrap().to_str().unwrap();

    if extension_map.contains_key(file_extension) {
        // get output path
        let mut output_path = file_path.to_owned();

        output_path.set_extension("md");

        // create LineWriter for output
        let out_file = File::create(&output_path).unwrap_or_else(|why| {
            panic!("Could not create outfile: {}", why);
        });
        let mut out_file = LineWriter::new(out_file);

        // write header
        let header = format!("```{}\n", extension_map.get(file_extension).unwrap());
        out_file.write_all(header.as_bytes());

        // read in
        let file = File::open(file_path).unwrap_or_else(|why| {
            panic!("Could not open {}: {}", file_path.to_str().unwrap(), why);
        });

        let mut buf_reader = BufReader::new(file);

        // TODO: error handling here
        for line in buf_reader.lines() {
            out_file.write_all(line.unwrap().as_bytes()).unwrap();
            out_file.write_all(b"\n").unwrap();
        }

        // write footer
        out_file.write_all(b"```");
        
        out_file.flush().unwrap();
    }
}

pub fn markdownify(mut input_path: PathBuf) {
    input_path = input_path.canonicalize().unwrap();

    let extension_map = get_map();

    if input_path.is_file() {
        file_to_markdown(&input_path, &extension_map);
    } else if input_path.is_dir() {
        find_files(&input_path, &extension_map);
    } else {
        // something is wrong??? is this condition possible?
    }
}

pub fn get_map() -> HashMap<String, String> {
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
        ("rs", "rust"),
        ("tex", "latex"),
    ];

    for ext in extensions.iter() {
        map_out.insert(ext.0.to_owned(), ext.1.to_owned());
    }

    map_out
}
