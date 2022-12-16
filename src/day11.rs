use std::fmt;

use itertools::Itertools;

use crate::read_lines::read_lines_unwrapped;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Monkey {
    id: usize,
    items: Vec<i32>,
    math: Math,
    test: i32,
    pass: usize,
    fail: usize,
    count: usize,
}

impl Monkey {
    fn new(id: usize, items: &Vec<i32>, math: Math, test: i32, pass: usize, fail: usize) -> Self {
        Monkey {
            id,
            items: items.clone(),
            math,
            test,
            pass,
            fail,
            count: 0,
        }
    }

    fn run(&mut self, part1: bool, common_multiple: i32) -> Box<Vec<(usize, i32)>> {
        let mut res: Box<Vec<(usize, i32)>> = Box::new(Vec::new());
        for old in self.items.drain(..) {
            let mut new = self.math.run(old);

            if part1 {
                new = new / 3;
            } else {
                new = new % common_multiple;
                dbg!(common_multiple, new);
            }

            let next;
            if new % self.test == 0 {
                next = self.pass;
            } else {
                next = self.fail;
            }

            res.push((next, new));
        }

        res
    }
}

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Monkey:{}<{:?}> '{}'", self.id, self.items, self.count)
    }
}

pub fn part1() {
    let mut monkeys: Vec<Monkey> = vec![];

    let input = read_lines_unwrapped("inputs/mine/day11.txt");
    let input = input.chunks(7);
    for mut chunk in &input {
        let id = parse_id(&mut chunk);
        let items = parse_items(&mut chunk);

        let operation = chunk.next().unwrap();
        let operation: Vec<_> = operation
            .strip_prefix("  Operation: new = ")
            .unwrap()
            .split(" ")
            .collect();
        let operation = Math::new(&operation);

        let test = chunk
            .next()
            .unwrap()
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let pass = chunk
            .next()
            .unwrap()
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let fail = chunk
            .next()
            .unwrap()
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        monkeys.push(Monkey::new(id, &items, operation, test, pass, fail));
    }

    for i in 0..20 {
        for j in 0..monkeys.len() {
            let x = monkeys[j].run(true, 0);
            for (from, value) in x.iter() {
                monkeys[j].count += 1;
                let to = &mut monkeys[*from].items;
                to.push(*value);
            }
        }
    }
    let mut res = monkeys.iter().map(|x| x.count).sorted().rev().take(2);
    let a = res.next().unwrap();
    let b = res.next().unwrap();

    dbg!(monkeys);
    println!("res: {}*{} = {}", a, b, a * b);
}

pub fn part2() {
    let mut monkeys: Vec<Monkey> = vec![];

    let input = read_lines_unwrapped("inputs/mine/day11.txt");
    let input = input.chunks(7);
    for mut chunk in &input {
        let id = parse_id(&mut chunk);
        let items = parse_items(&mut chunk);

        let operation = chunk.next().unwrap();
        let operation: Vec<_> = operation
            .strip_prefix("  Operation: new = ")
            .unwrap()
            .split(" ")
            .collect();
        let operation = Math::new(&operation);

        let test = chunk
            .next()
            .unwrap()
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let pass = chunk
            .next()
            .unwrap()
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let fail = chunk
            .next()
            .unwrap()
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        monkeys.push(Monkey::new(id, &items, operation, test, pass, fail));
    }

    let mut common_multiple = 1;
    for m in &monkeys {
        common_multiple *= m.test;
    }

    for i in 0..10000 {
        for j in 0..monkeys.len() {
            let x = monkeys[j].run(false, common_multiple);
            for (from, value) in x.iter() {
                monkeys[j].count += 1;
                let to = &mut monkeys[*from].items;
                to.push(*value);
            }
        }
    }
    let mut res = monkeys.iter().map(|x| x.count).sorted().rev().take(2);
    let a = res.next().unwrap();
    let b = res.next().unwrap();

    dbg!(monkeys);
    println!("res: {}*{} = {}", a, b, a * b);
}

fn parse_id(chunk: &mut itertools::Chunk<impl Iterator<Item = String>>) -> usize {
    let id = chunk.next().unwrap();
    let id = id
        .strip_prefix("Monkey ")
        .unwrap()
        .strip_suffix(":")
        .unwrap();
    let id = id.parse().unwrap();
    id
}

fn parse_items(chunk: &mut itertools::Chunk<impl Iterator<Item = String>>) -> Vec<i32> {
    let items = chunk.next();
    let items = items.unwrap();
    let items = items
        .strip_prefix("  Starting items: ")
        .unwrap()
        .split(", ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    items
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Math {
    a: Value,
    b: Value,
    op: Operation,
}

impl Math {
    fn new(input: &Vec<&str>) -> Self {
        let a = match *input.get(0).unwrap() {
            "old" => Value::OLD,
            x => Value::CONST(x.parse::<i32>().unwrap()),
        };

        let b = match *input.get(2).unwrap() {
            "old" => Value::OLD,
            x => Value::CONST(x.parse::<i32>().unwrap()),
        };

        let op = match *input.get(1).unwrap() {
            "*" => Operation::TIMES,
            "+" => Operation::PLUS,
            x => panic!("unknown op: {}", x),
        };

        Math { a, b, op }
    }

    fn run(&self, old: i32) -> i32 {
        match self.op {
            Operation::PLUS => self.a.get(old) + self.b.get(old),
            Operation::TIMES => self.a.get(old) * self.b.get(old),
        }
    }
}

impl Value {
    fn get(&self, old: i32) -> i32 {
        match self {
            Self::CONST(x) => *x,
            Self::OLD => old,
        }
    }
}

impl fmt::Debug for Math {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.a, self.op, self.b)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Operation {
    PLUS,
    TIMES,
}

impl fmt::Debug for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PLUS => write!(f, "+"),
            Self::TIMES => write!(f, "*"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Value {
    OLD,
    CONST(i32),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OLD => write!(f, "old"),
            Self::CONST(x) => write!(f, "{}", x),
        }
    }
}
