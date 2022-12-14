use std::{collections::HashSet, fmt};

use crate::read_lines::read_lines_unwrapped;

pub fn part1() {
    let input = read_lines_unwrapped("inputs/mine/day9.txt");
    let mut visited: HashSet<Pt> = HashSet::new();

    let mut tail = Pt { x: 0, y: 0 };
    let mut head = Pt { x: 0, y: 0 };

    visited.insert(tail);

    for line in input {
        let direction = line.chars().nth(0).unwrap();
        let distance = line.split(" ").nth(1).unwrap().parse::<i32>().unwrap();

        for _ in 0..distance {
            match direction {
                'R' => head.x += 1,
                'L' => head.x -= 1,
                'U' => head.y += 1,
                'D' => head.y -= 1,
                _ => panic!("uknonw direction"),
            }

            let d = pts_to_direction(head, tail);

            match d {
                Direction::None => {}
                Direction::R => tail.x += 1,
                Direction::U => tail.y += 1,
                Direction::L => tail.x -= 1,
                Direction::D => tail.y -= 1,
                Direction::UR => {
                    tail.x += 1;
                    tail.y += 1
                }
                Direction::UL => {
                    tail.y += 1;
                    tail.x -= 1
                }
                Direction::DL => {
                    tail.x -= 1;
                    tail.y -= 1
                }
                Direction::DR => {
                    tail.y -= 1;
                    tail.x += 1
                }
            };
            visited.insert(tail);
        }
    }
    print!("{}", visited.len());
}

fn pts_to_direction(a: Pt, b: Pt) -> Direction {
    let distance_x = a.x - b.x;
    let distance_y = a.y - b.y;
    match (distance_x, distance_y) {
        (x, y) if x > 2 || y > 2 => panic!(),
        (0, 2) => Direction::U,
        (0, -2) => Direction::D,
        (2, 0) => Direction::R,
        (-2, 0) => Direction::L,
        (1, -2) | (2, -1) => Direction::DR,
        (-1, -2) | (-2, -1) => Direction::DL,
        (1, 2) | (2, 1) => Direction::UR,
        (-1, 2) | (-2, 1) => Direction::UL,

        _ => Direction::None,
    }
}

#[derive(Clone, Debug, Copy)]
enum Direction {
    U,
    UR,
    R,
    DR,
    D,
    DL,
    L,
    UL,
    None,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pt {
    x: i32,
    y: i32,
}

impl fmt::Debug for Pt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pt<{},{}>", self.x.to_string(), self.y.to_string())
    }
}
