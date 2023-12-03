use std::fs::read_to_string;
use std::path::Path;

pub fn read_lines<T: AsRef<Path>>(pathname: T) -> Vec<String> {
    read_to_string(pathname)
        .expect("unable to open file")
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

pub fn read_str(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}
