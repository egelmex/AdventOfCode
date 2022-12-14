use crate::read_lines::read_lines_unwrapped;

const COL_WIDTH: isize = 40;

pub fn part1() {
    const CHECK: [usize; 6] = [20, 60, 100, 140, 180, 220];
    let input = read_lines_unwrapped("inputs/mine/day10.txt");

    let mut x: isize = 1;
    let mut c: usize = 0;
    let mut res = 0;
    for i in input {
        if i == "noop" {
            c += 1;
            if CHECK.contains(&c) {
                res += x * c as isize;
            }
        } else if i.starts_with("addx ") {
            let add: isize = i.strip_prefix("addx ").unwrap().parse::<isize>().unwrap();
            c += 1;
            if CHECK.contains(&c) {
                res += x * c as isize;
            }
            c += 1;
            if CHECK.contains(&c) {
                res += x * c as isize;
            }
            x += add;
        }

        //dbg!(x, c);
    }
    println!("{}", res);
}

pub fn part2() {
    let input = read_lines_unwrapped("inputs/mine/day10.txt");

    let mut x: isize = 1;
    let mut c: usize = 0;

    let mut screen = [' '; (40 * 6) + 1];
    screen[c] = if [x - 1, x, x + 1].contains(&(&(c as isize) % COL_WIDTH)) {
        '#'
    } else {
        ' '
    };

    for i in input {
        if i == "noop" {
            c += 1;
            screen[c] = if [x - 1, x, x + 1].contains(&(&(c as isize) % COL_WIDTH)) {
                '#'
            } else {
                ' '
            };
        } else if i.starts_with("addx ") {
            let add: isize = i.strip_prefix("addx ").unwrap().parse::<isize>().unwrap();
            c += 1;

            screen[c] = if [x - 1, x, x + 1].contains(&(&(c as isize) % COL_WIDTH)) {
                '#'
            } else {
                ' '
            };
            c += 1;

            x += add;
            screen[c] = if [x - 1, x, x + 1].contains(&(&(c as isize) % COL_WIDTH)) {
                '#'
            } else {
                ' '
            };
        }

        //dbg!(x, c);
    }

    let rows = screen.chunks(COL_WIDTH as usize);
    for row in rows {
        println!(" - {}", String::from_iter(row));
    }
}
