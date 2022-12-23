use std::fmt;

pub fn xy_to((x, y): (usize, usize), width: usize) -> usize {
    y * width + x
}

pub fn to_xy(i: usize, width: usize) -> (usize, usize) {
    let x = i % width;
    let y = i / width;

    (x, y)
}

#[derive(Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Result<Point, String> {
        Ok(Point { x, y })
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{},{}]>", self.x, self.y)
    }
}
