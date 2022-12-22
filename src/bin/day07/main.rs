use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::cmp::{Ord, Ordering, PartialOrd, PartialEq, Eq};

#[derive()]
struct Data {
    size : usize,
    is_folder : bool,
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        self.size.cmp(&other.size)
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
    }
}


impl Eq for Data { }

fn update_folder_sizes(file_size : usize, mut file_path : PathBuf, map : &mut HashMap<PathBuf, Data>) {
    while let Some(parent) = file_path.parent() {
        map.get_mut(parent).unwrap().size += file_size;
        file_path.pop();
    }
}

fn parse_input(s : &str) -> HashMap<PathBuf, Data> {
    let mut result = HashMap::new();
    let mut current_path = PathBuf::new();

    for command in s.split("$ ").skip(1) {
        match command.split_once(&[' ','\n']).unwrap() {
            ("cd", dir) => {
                if dir.starts_with("..") {
                    _ = current_path.pop();
                } else {
                    current_path.push(dir.split_once('\n').unwrap().0);
                }
                result.entry(current_path.clone()).or_insert(Data{size : 0, is_folder : true});
            },
            ("ls", list) => {
                for line in list.lines() {
                    if line.starts_with("dir") {
                        current_path.push(line.split_once(' ').unwrap().1);
                        result.entry(current_path.clone()).or_insert(Data{size : 0, is_folder : true});
                    } else {
                        let (size, file_name) = line.split_once(' ').unwrap();
                        current_path.push(file_name);
                        result.entry(current_path.clone()).or_insert(Data{size : size.parse().unwrap(), is_folder : false});
                        update_folder_sizes(size.parse().unwrap(), current_path.clone(), &mut result);
                    }
                    _ = current_path.pop();
                }
            },
            _ => unreachable!(),
        }
    }
    result
}

fn main() {
    let input = include_str!("input");
    let file_system = parse_input(input);
    let necessary_space = 30000000 - (70000000 - file_system.get(Path::new("/")).unwrap().size);
    println!("Output1: {}\nOutput2: {}", file_system.values()
        .filter(|data| data.is_folder && data.size <= 100000)
        .fold(0,|n, i| n + i.size), 
        file_system.values()
            .filter(|data| data.is_folder && data.size >= necessary_space)
            .min().unwrap().size);
}

#[test]
fn test_part1() {
    let test_string = "$ cd /\n\
        $ ls\n\
        dir a\n\
        14848514 b.txt\n\
        8504156 c.dat\n\
        dir d\n\
        $ cd a\n\
        $ ls\n\
        dir e\n\
        29116 f\n\
        2557 g\n\
        62596 h.lst\n\
        $ cd e\n\
        $ ls\n\
        584 i\n\
        $ cd ..\n\
        $ cd ..\n\
        $ cd d\n\
        $ ls\n\
        4060174 j\n\
        8033020 d.log\n\
        5626152 d.ext\n\
        7214296 k";

    assert_eq!(parse_input(test_string)
        .values()
        .filter(|data| data.is_folder && data.size <= 100000)
        .fold(0,|n, i| n + i.size), 95437);

}