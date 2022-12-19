pub fn xy_to((x, y): (usize, usize), width: usize) -> usize {
    y * width + x
}

pub fn to_xy(i: usize, width: usize) -> (usize, usize) {
    let x = i % width;
    let y = i / width;

    (x, y)
}
