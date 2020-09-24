use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn file_in(file_path: &str) -> Vec<String> {
    let mut v = vec![];
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(l) = line {
                v.push(l);
            }
        }
    }
    return v;
}

fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}
