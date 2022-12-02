use std::{
    fs::File,
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

pub fn day1() {
    let input = read_lines("day1.txt").expect("Failed to read file");
    let input = input.map(|x| -> Result<u32, _> { x.expect("err").parse() });
    let input: Vec<_> = input.collect();

    let mut all: Vec<u32> = vec![];
    let mut count = 0;
    for a in input {
        match a {
            Ok(x) => count += x,
            _ => {
                all.push(count);
                count = 0;
            }
        }
    }
    all.push(count);

    all.sort_unstable();
    all.reverse();

    let top = &all[0..3];
    let tot: u32 = top.iter().sum();
    print!("{tot}");
}
