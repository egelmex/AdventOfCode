use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
};

use crate::read_lines;

pub fn part1() {
    let mut lines = read_lines::read_lines_unwrapped("inputs/mine/day8.txt").peekable();
    let width = lines.peek().unwrap().len();

    println!("{width}");

    let mut grid: Vec<RefCell<Tree>> = vec![
        RefCell::new(Tree {
            height: 0,
            visible: false,
            view: 0,
        });
        width * width
    ];

    for (i, line) in lines.enumerate() {
        for (j, height) in line.chars().enumerate() {
            let height = height.to_string().parse::<usize>().unwrap();
            grid[(i * width) + j].borrow_mut().height = height;

            grid[(i * width) + j].borrow_mut().visible =
                i == 0 || i == width - 1 || j == 0 || j == width - 1;
        }
    }

    for i in 0..width * width {
        let current = &grid[i];
        let (x, y) = to_xy(i, width);

        let mut t1: usize = 0;
        for lx in 0..x {
            t1 = t1.max(grid[xy_to((lx, y), width)].borrow().height);
        }
        let mut t2: usize = 0;
        for lx in x + 1..width {
            t2 = t2.max(grid[xy_to((lx, y), width)].borrow().height);
        }

        let mut t3: usize = 0;
        for ly in 0..y {
            t3 = t3.max(grid[xy_to((x, ly), width)].borrow().height);
        }
        let mut t4: usize = 0;
        for ly in y + 1..width {
            t4 = t4.max(grid[xy_to((x, ly), width)].borrow().height);
        }

        let min = t1.min(t2).min(t3).min(t4);

        if current.borrow().height > min {
            current.borrow_mut().visible = true;
        }
    }
    print(&grid, width);
    let count: usize = grid.iter().filter(|x| (*x).borrow().visible).count();
    dbg!(count);
}

pub fn part2() {
    let mut lines = read_lines::read_lines_unwrapped("inputs/mine/day8.txt").peekable();
    let width = lines.peek().unwrap().len();

    println!("{width}");

    let mut grid: Vec<RefCell<Tree>> = vec![
        RefCell::new(Tree {
            height: 0,
            visible: false,
            view: 0,
        });
        width * width
    ];

    for (i, line) in lines.enumerate() {
        for (j, height) in line.chars().enumerate() {
            let height = height.to_string().parse::<usize>().unwrap();
            grid[(i * width) + j].borrow_mut().height = height;

            grid[(i * width) + j].borrow_mut().visible =
                i == 0 || i == width - 1 || j == 0 || j == width - 1;
        }
    }

    for i in 0..width * width {
        let current = &grid[i];
        let current_height = current.borrow().height;
        let (x, y) = to_xy(i, width);

        let mut v1: usize = 0;
        for lx in (0..x).rev() {
            v1 += 1;
            if current_height <= grid[xy_to((lx, y), width)].borrow().height {
                break;
            }
        }
        let mut v2: usize = 0;
        for lx in (x + 1)..width {
            v2 += 1;
            if current_height <= grid[xy_to((lx, y), width)].borrow().height {
                break;
            }
        }
        let mut v3: usize = 0;
        for ly in (0..y).rev() {
            v3 += 1;
            if current_height <= grid[xy_to((x, ly), width)].borrow().height {
                break;
            }
        }
        let mut v4: usize = 0;
        for ly in (y + 1)..width {
            v4 += 1;
            if current_height <= grid[xy_to((x, ly), width)].borrow().height {
                break;
            }
        }

        current.borrow_mut().view = v1 * v2 * v3 * v4;
    }
    print_view(&grid, width);

    let mut best = 0;
    for i in 0..(width * width) {
        if grid[i].borrow().view > best {
            best = grid[i].borrow().view;
        }
    }

    dbg!(best);
}

fn xy_to((x, y): (usize, usize), width: usize) -> usize {
    y * width + x
}

fn to_xy(i: usize, width: usize) -> (usize, usize) {
    let x = i % width;
    let y = i / width;

    (x, y)
}

fn print(grid: &Vec<RefCell<Tree>>, width: usize) {
    for line in grid.chunks(width) {
        for c in line {
            print!(
                "{} {} - ",
                c.borrow().height,
                if c.borrow().visible { "t" } else { "f" }
            );
        }
        println!("");
    }
    println!("");
}

fn print_view(grid: &Vec<RefCell<Tree>>, width: usize) {
    for line in grid.chunks(width) {
        for c in line {
            print!("h{} v{} - ", c.borrow().height, c.borrow().view,);
        }
        println!("");
    }
    println!("");
}

#[derive(Clone, Debug, Copy)]
struct Tree {
    height: usize,
    visible: bool,
    view: usize,
}
