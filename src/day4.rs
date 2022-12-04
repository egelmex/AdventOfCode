use std::cmp;

use crate::read_lines;

pub fn part1() {
    let input = read_lines::read_lines("day4.txt")
        .expect("falied to read")
        .map(|x| x.unwrap())
        .map(|x| line_to_pair(&x))
        .map(|x| x.0.contains(&x.1))
        .filter(|x| *x)
        .count();

    println!("{:?}", input);
}

pub fn part2() {
    let input = read_lines::read_lines("day4.txt")
        .expect("falied to read")
        .map(|x| x.unwrap())
        .map(|x| line_to_pair(&x))
        .map(|x| x.0.overlaps(&x.1))
        .filter(|x| *x)
        .count();
    println!("{:?}", input);
}

struct Range {
    start: u32,
    end: u32,
}

fn line_to_pair(line: &str) -> (Range, Range) {
    let mut parts = line.split(",");

    let a = parts.next().expect("no first");
    let b = parts.next().expect("no second");
    (Range::from(a), Range::from(b))
}

impl From<&str> for Range {
    fn from(input: &str) -> Self {
        // 2-4
        let mut parts = input.split("-");
        let start = parts.next().expect("no start").parse().expect("no an int");
        let end = parts.next().expect("no end").parse().expect("not an int");

        Range::new(start, end)
    }
}

impl Range {
    fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    fn contains(&self, other: &Self) -> bool {
        (other.start >= self.start && other.end <= self.end)
            || (self.start >= other.start && self.end <= other.end)
    }

    fn overlaps(&self, other: &Self) -> bool {
        (self.start >= other.start && self.start <= other.end)
            || (other.start >= self.start && other.start <= self.end)
    }

    /// I miss read the instructions...
    #[allow(dead_code)]
    fn count_overlap(&self, other: &Self) -> u32 {
        cmp::max(
            0,
            cmp::min(self.end as i32, other.end as i32)
                - cmp::max(self.start as i32, other.start as i32),
        ) as u32
    }
}
