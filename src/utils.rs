use std::path::{Path, PathBuf};

pub fn find_files(input_path: &PathBuf) {
    let mut dirs_to_search: Vec<PathBuf> = vec![input_path.to_path_buf()];
    
    while !dirs_to_search.is_empty() {
        let current_dir = dirs_to_search.pop().unwrap();
        
        for entry in current_dir.read_dir().expect("read_dir call failure") {
            if let Ok(entry) = entry {
                let entry_path = entry.path();

                if entry_path.is_file() {
                    file_to_markdown(&entry_path);
                } else if entry_path.is_dir() {
                    dirs_to_search.push(entry_path);
                }
            }
        }
    }
}

pub fn file_to_markdown(file_path: &PathBuf) {
    println!("{:?}", file_path);
}

pub fn markdownify(mut input_path: PathBuf) {
    input_path = input_path.canonicalize().unwrap();

    if input_path.is_file() {
        file_to_markdown(&input_path);
    } else if input_path.is_dir() {
        find_files(&input_path); 
    } else {
        // something is wrong??? is this condition possible?
    }

    

}
