use std::vec;

use crate::read_lines::read_as_string;
use crate::util::Point;

peg::parser! {
    grammar list_parser() for str {
        rule value() -> usize
        = x: $(['0'..='9']+) {? x.parse().or(Err("u32"))}

        rule point() -> Point
        = x:value() "," y:value() {? Point::new(x,y).or(Err("point")) }

        rule list() -> Vec<Point>
        =  l:point() ** " -> " {? (Ok(l)) }

        pub rule document() -> Vec<Vec<Point>>
        = l:list() ** "\n" {? (Ok(l))}

    }
}

pub fn part1() {
    let lines = read_as_string("inputs/mine/day14.txt");
    let input = list_parser::document(&lines).expect("failed to parse");
    let (min, max) = find_min_max(&input, Point { x: 500, y: 0 });

    let width = max.x - min.x + 1;
    let height = max.y - min.y + 1;

    let mut grid = vec!['.'; width * height];

    draw_lines(input, &mut grid, &min, width);

    let mut count = 0;
    loop {
        let ret = || -> Result<(), String> {
            let mut new_sand = Point::new(500 - min.x, 0).unwrap();
            loop {
                let below = new_sand.clone().mov(Direction::DOWN, height, width)?;
                let b = grid[below.x + below.y * width];
                if b == '.' {
                    new_sand.y = new_sand.y + 1;
                } else if b == '#' || b == 'o' {
                    let left = new_sand.clone().mov(Direction::LEFT, height, width)?;
                    let l = grid[left.x + left.y * width];

                    if l == '.' {
                        new_sand.y = new_sand.y + 1;
                        new_sand.x = new_sand.x - 1;
                        continue;
                    }

                    let right = new_sand.clone().mov(Direction::RIGHT, height, width)?;
                    let r = grid[right.x + right.y * width];
                    if r == '.' {
                        new_sand.y = &new_sand.y + 1;
                        new_sand.x = &new_sand.x + 1;
                        continue;
                    }

                    grid[&new_sand.x + &new_sand.y * width] = 'o';
                    break;
                }
            }

            //print_grid(&grid, min.clone(), width, height);
            count += 1;

            Ok(())
        }();
        match ret {
            Ok(_) => {}
            Err(_) => break,
        }
    }

    println!("{count}");
}

fn draw_lines(input: Vec<Vec<Point>>, grid: &mut Vec<char>, min: &Point, width: usize) {
    for rule in input {
        for pair in rule.windows(2) {
            let a = pair[0].clone();
            let b = pair[1].clone();
            for x in a.x.min(b.x)..=a.x.max(b.x) {
                for y in a.y.min(b.y)..=a.y.max(b.y) {
                    grid[x - min.x + y * width] = '#';
                }
            }
        }
    }
}

fn print_grid(grid: &Vec<char>, zero: Point, width: usize, height: usize) {
    let grid = grid.clone();
    let rows = grid.chunks(width);

    print!("  ");
    for i in zero.x..zero.x + width {
        print!("{i:4} ");
    }
    println!();
    for (i, row) in rows.enumerate() {
        print!("{i:^4}");
        for c in row {
            print!("{c:5}")
        }
        println!();
    }
}

fn find_min_max(input: &Vec<Vec<Point>>, source: Point) -> (Point, Point) {
    let input = input.clone().into_iter().flatten();
    let input = input.collect::<Vec<Point>>();
    let min = input
        .clone()
        .into_iter()
        .reduce(|p1, p2| Point::new(p1.x.min(p2.x), p1.y.min(p2.y)).unwrap())
        .unwrap();

    let max = input
        .into_iter()
        .reduce(|p1, p2| Point::new(p1.x.max(p2.x), p1.y.max(p2.y)).unwrap())
        .unwrap();

    (
        Point::new(min.x.min(source.x), min.y.min(source.y)).unwrap(),
        Point::new(max.x.max(source.x), max.y.max(source.y)).unwrap(),
    )
}

enum Direction {
    DOWN,
    LEFT,
    RIGHT,
}

impl Point {
    fn mov(self, direction: Direction, height: usize, width: usize) -> Result<Point, &'static str> {
        let mut res = self.clone();
        match direction {
            Direction::DOWN => res.y += 1,
            Direction::LEFT => {
                res.y += 1;
                if res.x == 0 {
                    return Err("Too Left");
                }
                res.x -= 1;
            }
            Direction::RIGHT => {
                res.y += 1;
                res.x += 1;
            }
        }

        if res.y >= height {
            return Err("Too Deep");
        }

        if res.x > width {
            return Err("Too Right");
        }

        Ok(res)
    }
}
