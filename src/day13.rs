use itertools::Itertools;
use std::fmt;

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

        //dbg!(&left, &right);
        let res = check(&left, &right);
        count += match res {
            RESULT::OK => i,
            RESULT::FAIL => 0,
            RESULT::CONTINUE => panic!("this shoudlnt be possible"),
        };

        dbg!(i, res);
    }
    println!("{}", count);
}

fn check(a: &V, b: &V) -> RESULT {
    dbg!(a, b);
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
            if (a.len() == b.len()) {
                return RESULT::CONTINUE;
            } else if a.len() < b.len() {
                return RESULT::OK;
            } else {
                return RESULT::FAIL;
            }
        }
    }
}
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
