use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    Vec::from_iter(lines.into_iter().map(|l| l.unwrap()))
}
