use crate::read_lines;
use std::collections::HashSet;

pub fn part1() {
    let prios: u32 = read_lines::read_lines("inputs/mine/day3.txt")
        .expect("failed to read file")
        .map(|x| x.unwrap())
        .map(|l| check_bag(l.chars().collect()))
        .map(|s| priority(s))
        .sum();

    println!("{:?}", prios);
}

pub fn part2() {
    let lines: Vec<HashSet<char>> = read_lines::read_lines("inputs/mine/day3.txt")
        .expect("failed to read file")
        .map(|x| x.unwrap())
        .map(|x| bag_to_set(&x))
        .collect();

    let x: u32 = lines.chunks(3).into_iter().map(|x| find_and_score(x)).sum();

    println!("{:?}", x);
}

fn find_and_score(s: &[HashSet<char>]) -> u32 {
    let intersection = s.iter().skip(1).fold(s[0].clone(), |acc, hs| {
        acc.intersection(hs).cloned().collect()
    });

    let c = intersection.iter().next().expect("fail");
    priority(*c)
}

fn bag_to_set(s: &String) -> HashSet<char> {
    HashSet::from_iter(s.chars())
}

fn check_bag(input: Vec<char>) -> char {
    let mut set_a: HashSet<&char> = HashSet::new();
    let mut set_b: HashSet<&char> = HashSet::new();
    let half = input.len() / 2;
    for c in &input[..half] {
        set_a.insert(c);
    }

    for c in &input[half..] {
        set_b.insert(c);
    }

    let mut i = set_a.intersection(&set_b);
    *i.next().expect("no match").clone()
}

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => panic!("unkown letter"),
    }
}
