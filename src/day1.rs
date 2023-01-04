use crate::read_lines;

pub fn day1() {
    let input = read_lines::read_lines_unwrapped("inputs/mine/day1.txt").map(|x| x.parse::<u32>());

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

    let tot: u32 = all[0..3].iter().sum();
    println!("{tot}");
}
