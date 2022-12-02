use crate::read_lines;

pub fn day2_part1() {
    let input = read_lines::read_lines("day2.txt")
        .expect("Failed to read file")
        .map(|x| x.unwrap())
        .map(|s| str_to_rps(s));

    let scores = input.map(|(a, b)| a.score(&b));
    let scores = scores.collect::<Vec<u32>>();
    let scores: u32 = scores.iter().sum();
    println!("{:?}", scores);
}

pub fn day2_part2() {
    let input = read_lines::read_lines("day2.txt")
        .expect("Failed to read file")
        .map(|x| x.unwrap())
        .map(|s| str_to_rps2(s));

    let scores = input.map(|(a, b)| a.score(&b));
    let scores = scores.collect::<Vec<u32>>();
    let scores: u32 = scores.iter().sum();
    println!("{:?}", scores);
}

#[derive(Debug, Copy, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn value(&self) -> u32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    fn score(&self, other: &RPS) -> u32 {
        let o = (other.value() + 1) as i32;
        let s = self.value() as i32;
        let res = (o - s).rem_euclid(3) * 3;
        res as u32
    }

    fn choose(&self, state: &str) -> RPS {
        let pick = match state {
            "X" => self.value() - 1,
            "Y" => self.value(),
            "Z" => self.value() + 1,
            _ => panic!("unknown"),
        };

        value_to_rps(pick)
    }
}

fn value_to_rps(value: u32) -> RPS {
    match value {
        0 => RPS::Scissors,
        1 => RPS::Rock,
        2 => RPS::Paper,
        3 => RPS::Scissors,
        4 => RPS::Rock,
        _ => panic!("unknown value : {value}"),
    }
}

fn str_to_rps(s: String) -> (RPS, RPS) {
    let tmp: Vec<_> = s.split(" ").collect();
    (parse_rps(tmp[0]), parse_rps(tmp[1]))
}

fn str_to_rps2(s: String) -> (RPS, RPS) {
    let tmp: Vec<_> = s.split(" ").collect();
    let a = parse_rps(tmp[0]);
    (a, a.choose(tmp[1]))
}

fn parse_rps(a: &str) -> RPS {
    match a {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissors,
        "X" => RPS::Rock,
        "Y" => RPS::Paper,
        "Z" => RPS::Scissors,
        _ => panic!("Wrong a"),
    }
}
