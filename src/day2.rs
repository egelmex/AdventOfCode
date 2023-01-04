use crate::read_lines;

pub fn day2_part1() {
    let scores: i32 = read_lines::read_lines_unwrapped("inputs/mine/day2.txt")
        .map(|s| str_to_rps(s))
        .map(|(a, b)| a.score(&b))
        .sum();

    println!("{:?}", scores);
}

pub fn day2_part2() {
    let scores: i32 = read_lines::read_lines_unwrapped("inputs/mine/day2.txt")
        .map(|s| str_to_rps2(s))
        .map(|(a, b)| a.score(&b))
        .sum();

    println!("{:?}", scores);
}

#[derive(Debug, Copy, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn value(&self) -> i32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    fn score(&self, other: &RPS) -> i32 {
        (other.value() + 1 - self.value()).rem_euclid(3) * 3
    }

    fn choose(&self, state: &str) -> RPS {
        value_to_rps(match state {
            "X" => self.value() - 1,
            "Y" => self.value(),
            "Z" => self.value() + 1,
            _ => panic!("unknown"),
        })
    }

    fn parse(a: &str) -> Self {
        match a {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scissors,
            _ => panic!("Wrong a"),
        }
    }
}

fn value_to_rps(value: i32) -> RPS {
    match value {
        0 | 3 => RPS::Scissors,
        1 | 4 => RPS::Rock,
        2 => RPS::Paper,
        _ => panic!("unknown value : {value}"),
    }
}

fn str_to_rps(s: String) -> (RPS, RPS) {
    let tmp: Vec<_> = s.split(" ").collect();
    (RPS::parse(tmp[0]), RPS::parse(tmp[1]))
}

fn str_to_rps2(s: String) -> (RPS, RPS) {
    let tmp = s.split(" ").collect::<Vec<_>>();
    let a = RPS::parse(tmp[0]);
    (a, a.choose(tmp[1]))
}
