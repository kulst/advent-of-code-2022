use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror;

struct Data {
    size : usize,
    is_folder : bool,
}

fn update_folder_sizes(file_size : usize, file_path : &Path, map : &mut HashMap<PathBuf, Data>) {
    while let Some(parent) = file_path.parent() {
        map.get_mut(parent).unwrap().size += file_size;
    }
}

fn parse_input(s : &str) -> HashMap<PathBuf, Data> {

}