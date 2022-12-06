use crate::read_lines;

pub fn part1() {
    let input = read_lines::read_lines("inputs/mine/day5.txt")
        .unwrap()
        .map(|x| x.unwrap());

    let mut crates: Vec<String> = vec![];
    let mut commands: Vec<String> = vec![];
    let mut a = true;
    for line in input {
        match a {
            true => {
                if line.is_empty() {
                    a = false;
                } else {
                    crates.push(line);
                }
            }

            _ => commands.push(line),
        }
    }
    let mut cols = process_crates(crates);
    for instruction in commands {
        let p = instruction
            .split(" ")
            .map(|x| x.parse::<usize>().unwrap_or_default())
            .collect::<Vec<_>>();
        let m: usize = p[1];
        let f: usize = p[3];
        let t: usize = p[5];

        let mut tmp: Vec<char> = vec![];
        for _ in 0..m {
            tmp.push(cols[f - 1].pop().unwrap());
        }
        for i in tmp {
            cols[t - 1].push(i);
        }
    }

    let mut out: String = "".to_string();
    for mut i in cols {
        out.push_str(i.pop().unwrap().to_string().as_str());
    }
    println!("{out}");
}

pub fn part2() {
    let input = read_lines::read_lines("inputs/mine/day5.txt")
        .unwrap()
        .map(|x| x.unwrap());

    let mut crates: Vec<String> = vec![];
    let mut commands: Vec<String> = vec![];
    let mut a = true;
    for line in input {
        match a {
            true => {
                if line.is_empty() {
                    a = false;
                } else {
                    crates.push(line);
                }
            }

            _ => commands.push(line),
        }
    }
    let mut cols = process_crates(crates);
    for instruction in commands {
        let p = instruction
            .split(" ")
            .map(|x| x.parse::<usize>().unwrap_or_default())
            .collect::<Vec<_>>();
        let m: usize = p[1];
        let f: usize = p[3];
        let t: usize = p[5];

        let mut tmp: Vec<char> = vec![];
        for _ in 0..m {
            tmp.push(cols[f - 1].pop().expect("fail"));
        }
        tmp.reverse();
        for i in tmp {
            cols[t - 1].push(i);
        }
    }

    let mut out: String = "".to_string();
    for mut i in cols {
        out.push_str(i.pop().expect("fail").to_string().as_str());
    }
    println!("{out}");
}

fn process_crates(lines: Vec<String>) -> Vec<Vec<char>> {
    let mut lines = lines.clone();
    let names = lines.pop().expect("failed");

    let col_count = get_column_count(names);

    let mut columns: Vec<Vec<char>> = vec![];
    for _ in 0..col_count {
        columns.push(vec![]);
    }

    lines.reverse();
    for line in lines {
        let chars = line.chars().collect::<Vec<_>>();
        for i in 0..col_count {
            let c = chars[(i * 4) + 1];
            if c.is_alphabetic() {
                columns[i as usize].push(c);
            }
        }
    }

    return columns;
}

fn get_column_count(s: String) -> usize {
    s.split(" ")
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>()
        .pop()
        .unwrap()
        .parse::<usize>()
        .unwrap()
}
