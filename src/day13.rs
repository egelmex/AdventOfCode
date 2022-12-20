use itertools::Itertools;
use std::{cmp::Ordering, fmt};

use crate::read_lines::read_lines_unwrapped;

peg::parser! {
    grammar list_parser() for str {
        rule number() -> V
        = n:$(['0'..='9']+) {? V::new_value(n.parse().or(Err("u32"))) }

        pub rule list() -> V
        = "[" l:((number()/list()) ** ",") "]" {? V::new_list(Ok(l)) }
    }
}

pub fn part1() {
    let lines = read_lines_unwrapped("inputs/mine/day13.txt").chunks(3);

    let mut count = 0;
    let mut i = 0;
    for mut c in &lines {
        i += 1;
        let left = c.next().unwrap();
        let right = c.next().unwrap();

        let mut left = list_parser::list(&left).unwrap();
        let mut right = list_parser::list(&right).unwrap();

        let res = check(&left, &right);
        count += match res {
            RESULT::OK => i,
            RESULT::FAIL => 0,
            RESULT::CONTINUE => panic!("this shoudlnt be possible"),
        };
    }
    println!("{}", count);
}

pub fn part2() {
    let lines = read_lines_unwrapped("inputs/mine/day13.txt");

    let mut parsed: Vec<V> = vec![];

    for line in lines {
        if line.is_empty() {
            continue;
        }
        parsed.push(list_parser::list(&line).unwrap());
    }

    let a = list_parser::list("[[2]]").unwrap();
    parsed.push(a.clone());
    let b = list_parser::list("[[6]]").unwrap();
    parsed.push(b.clone());

    parsed.sort_by(|a, b| order(a, b));

    let mut res = 1;
    for (i, v) in parsed.iter().enumerate() {
        if v == &a || v == &b {
            res *= i + 1;
        }
    }
    println!("{res}");
}

fn check(a: &V, b: &V) -> RESULT {
    match (a, b) {
        (V::Value(a), V::Value(b)) if a < b => RESULT::OK,
        (V::Value(a), V::Value(b)) if a > b => RESULT::FAIL,
        (V::Value(_), V::Value(_)) => RESULT::CONTINUE,
        (V::List(_), V::Value(b)) => check(a, &V::List(vec![V::Value(*b)])),
        (V::Value(a), V::List(_)) => check(&V::List(vec![V::Value(*a)]), b),
        (V::List(a), V::List(b)) => {
            for items in a.iter().zip(b) {
                match check(items.0, items.1) {
                    RESULT::FAIL => return RESULT::FAIL,
                    RESULT::OK => return RESULT::OK,
                    RESULT::CONTINUE => continue,
                };
            }
            if a.len() == b.len() {
                return RESULT::CONTINUE;
            } else if a.len() < b.len() {
                return RESULT::OK;
            } else {
                return RESULT::FAIL;
            }
        }
    }
}
#[derive(Clone, Hash, Eq, PartialEq)]
pub enum V {
    Value(u32),
    List(Vec<V>),
}

impl V {
    fn new_value(v: Result<u32, &str>) -> Result<Self, &str> {
        match v {
            Err(x) => Err(x),
            Ok(x) => Ok(V::Value(x)),
        }
    }

    fn new_list(v: Result<Vec<V>, &str>) -> Result<Self, &str> {
        match v {
            Err(x) => Err(x),
            Ok(x) => Ok(V::List(x)),
        }
    }
}

impl fmt::Debug for V {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            V::List(x) => write!(f, "{:?}", x),
            V::Value(x) => write!(f, "{}", x),
        }
    }
}
#[derive(Debug)]
enum RESULT {
    CONTINUE,
    FAIL,
    OK,
}

fn order(a: &V, b: &V) -> Ordering {
    match check(&a, &b) {
        RESULT::OK => Ordering::Less,
        RESULT::FAIL => Ordering::Greater,
        RESULT::CONTINUE => Ordering::Equal,
    }
}
