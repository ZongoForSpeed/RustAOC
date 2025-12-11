use std::path::Path;

pub fn read_lines(path: &Path) -> Vec<String> {
    std::fs::read_to_string(path)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
