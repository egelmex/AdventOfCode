use crate::read_lines;

pub fn day1() {
    let input = read_lines::read_lines("inputs/mine/day1.txt").expect("Failed to read file");
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
    println!("{tot}");
}
