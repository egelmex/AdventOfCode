use std::fmt;

use crate::read_lines::read_lines_unwrapped;
use crate::util;
use pathfinding::prelude::astar;

pub fn part1() {
    let (height, width, grid, start, end) = parse_input();

    let end = end.unwrap();
    let res = astar(
        &start.unwrap(),
        |x| successors(x, &grid, width, height),
        |p| huristic(p, &end),
        |p| test(p, &end),
    )
    .expect("no path found");

    println!("{:?}", res.1);
}

pub fn part2() {
    let (height, width, grid, start, end) = parse_input();

    let mut min = 9999;
    let start = start.unwrap();
    let end = end.unwrap();
    for (i, c) in grid.iter().enumerate() {
        if *c == 0 {
            let (x, y) = util::to_xy(i, width);
            let start = Pos { x: x, y: y };
            match astar(
                &start,
                |x| successors(x, &grid, width, height),
                |p| huristic(p, &end),
                |p| test(p, &end),
            ) {
                None => {}
                Some(x) => min = min.min(x.1),
            }
        }
    }

    println!("{}", min);
}

fn parse_input() -> (usize, usize, Vec<u32>, Option<Pos>, Option<Pos>) {
    let input = read_lines_unwrapped("inputs/mine/day12.txt").collect::<Vec<_>>();
    let mut input = input.iter().peekable();
    let height = input.len();
    let width = input.peek().unwrap().len();
    let mut grid = vec![0; height * width];
    let mut start = None;
    let mut end = None;
    for (line_i, line) in input.enumerate() {
        for (c_i, c) in line.chars().enumerate() {
            grid[line_i * width + c_i] = match c {
                'S' => {
                    start = Some(Pos { x: c_i, y: line_i });
                    9999
                }
                'E' => {
                    end = Some(Pos { x: c_i, y: line_i });
                    9999
                }
                x => {
                    let x = x as u32;
                    let a = 'a' as u32;
                    x - a
                }
            }
        }
    }
    assert!(
        grid.len() == width * height,
        "{} != {}",
        grid.len(),
        width * height
    );
    (height, width, grid, start, end)
}

fn test(pt: &Pos, end: &Pos) -> bool {
    pt == end
}

fn huristic(pt: &Pos, end: &Pos) -> u32 {
    1
    //(pt.x.abs_diff(end.x) + pt.y.abs_diff(end.y)) as u32
}

fn successors(pt: &Pos, grid: &Vec<u32>, width: usize, g_height: usize) -> Vec<(Pos, u32)> {
    let mut next: Vec<(Pos, u32)> = vec![];
    let current_height = grid[util::xy_to((pt.x, pt.y), width)];
    if pt.y > 0 {
        let index = util::xy_to((pt.x, pt.y - 1), width);
        let tmp_height = grid[index];
        if tmp_height <= current_height + 1 || current_height == 9999 || tmp_height == 9999 {
            next.push((
                Pos {
                    x: pt.x,
                    y: pt.y - 1,
                },
                1,
            ));
        }
    }

    if pt.x > 0 {
        let index = util::xy_to((pt.x - 1, pt.y), width);
        let tmp_height = grid[index];
        if tmp_height <= current_height + 1 || current_height == 9999 || tmp_height == 9999 {
            next.push((
                Pos {
                    x: pt.x - 1,
                    y: pt.y,
                },
                1,
            ));
        }
    }

    if pt.x < width - 1 {
        let index = util::xy_to((pt.x + 1, pt.y), width);
        let tmp_height = grid[index];
        if tmp_height <= (current_height + 1) || current_height == 9999 || tmp_height == 9999 {
            next.push((
                Pos {
                    x: pt.x + 1,
                    y: pt.y,
                },
                1,
            ));
        }
    }

    if pt.y < g_height - 1 {
        let index = util::xy_to((pt.x, pt.y + 1), width);
        let tmp_height = grid[index];
        if tmp_height <= (current_height + 1) || current_height == 9999 || tmp_height == 9999 {
            next.push((
                Pos {
                    x: pt.x,
                    y: pt.y + 1,
                },
                1,
            ));
        }
    }

    next
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn _distance(&self, _other: &Pos) -> u32 {
        1
    }
}

impl fmt::Debug for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{},{}>'", self.x, self.y)
    }
}
