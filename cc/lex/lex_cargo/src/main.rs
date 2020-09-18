use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
mod lex;

//file io
fn file_in(file_path: &str) -> Vec<String> {
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let to_lex: Vec<String> = file_in("src/test_files/return_2.c");

    for s in &to_lex {
        println!("{}", s);
    }

    let lexed: Vec<u8>;
    lexed = lex::lex(to_lex);

    for i in lexed {
        println!("{}", i);
    }
}
