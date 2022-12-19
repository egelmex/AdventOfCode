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
    let lines = read_lines_unwrapped("inputs/examples/day13.txt").chunks(3);

    for mut c in &lines {
        let left = c.next().unwrap();
        let right = c.next().unwrap();

        let mut left = list_parser::list(&left).unwrap();
        let mut right = list_parser::list(&right).unwrap();

        dbg!(left, right);
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
