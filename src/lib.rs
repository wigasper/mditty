pub mod utils;

#[cfg(test)]
mod tests {
    pub use crate::utils::*;

    use std::collections::{HashMap, HashSet};
    use std::path::{Path, PathBuf};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn t1() {
        let map = get_map();
        let file_path = PathBuf::from("test.rs");
        let file_extension = file_path.extension().unwrap().to_str().unwrap();
        assert!(map.contains_key(file_extension));
    }

    #[test]
    fn t2() {
        let mut p = PathBuf::from("/this/is/a/test.rs");
        p.set_extension("");
        assert_eq!(Path::new("/this/is/a/test"), p.as_path());
    }
    //    #[test]
    //    fn test_get_extensions() {
    //        let ext_map = get_map();
    //        let extensions: HashSet<String> = get_extensions();
    //
    //       for key in ext_map.keys() {
    //           assert!(extensions.contains(&key.to_owned()));
    //        }
    //    }
}
