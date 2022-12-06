use std::{
    fs::{self, File},
    io::{self, BufRead},
    path::Path,
};

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_lines_unwrapped(filename: &str) -> impl Iterator<Item = String> {
    read_lines(filename).unwrap().map(|x| x.unwrap())
}

pub fn read_as_string<P>(filename: P) -> String
where
    P: AsRef<Path>,
{
    fs::read_to_string(filename).unwrap()
}
