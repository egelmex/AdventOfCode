use crate::read_lines;

pub fn part1() {
    let input = read_lines::read_as_string("inputs/mine/day6.txt");
    let buffer = Buffer::new(4);
    match check_all(input, buffer) {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e),
    }
}

pub fn part2() {
    let input = read_lines::read_as_string("inputs/mine/day6.txt");
    let buffer = Buffer::new(14);
    match check_all(input, buffer) {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e),
    }
}

fn check_all(input: String, mut buffer: Buffer) -> Result<usize, &'static str> {
    for ch in input.chars() {
        if buffer.test(ch) {
            return Ok(buffer.count);
        }
        buffer.write(ch);
    }
    Err("No message found")
}

#[derive(Debug)]
struct Buffer {
    data: Vec<char>,
    pointer: usize,
    count: usize,
    size: usize,
}

impl Buffer {
    fn test(&self, data: char) -> bool {
        match self.count {
            0..=3 => false,
            _ => !has_dup(&self.data),
        }
    }

    fn write(&mut self, to_write: char) {
        self.data[self.pointer] = to_write;
        self.pointer = (self.pointer + 1) % self.size;
        self.count = self.count + 1;
    }

    fn new(size: usize) -> Self {
        Self {
            data: vec!['.'; size],
            pointer: 0,
            count: 0,
            size,
        }
    }
}

fn has_dup<T: PartialEq>(slice: &[T]) -> bool {
    for i in 1..slice.len() {
        if slice[i..].contains(&slice[i - 1]) {
            return true;
        }
    }
    false
}
