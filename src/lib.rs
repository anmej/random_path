#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct PathBuilder {
    width: usize,
    heigth: usize,
    start: Point,
    end: Point,
    blacklist: Vec<Point>, // list of locations that lead to dead end
    path: Vec<Point>,
}

impl PathBuilder {
    pub fn new(width: usize, heigth: usize, start: Point, end: Point) -> PathBuilder {
        PathBuilder {
            width: width,
            heigth: heigth,
            start: start,
            end: end,
            blacklist: vec![],
            path: vec![],
        }
    }

    pub fn get_free_neighbours(point: Point) -> Vec<Point> {
        for (x, y) in [1, 1, -1, -1].iter().zip([1, -1, -1, 1].iter()) {
            
        }
        vec!()
    }
}
pub fn manhattan_distance(a: Point, b: Point) -> usize {
    ((a.x as isize - b.x as isize).abs() + (a.y as isize - b.y as isize).abs()) as usize

}

pub fn is_neighbours(a: Point, b: Point) -> bool {
    manhattan_distance(a, b) < 3
}


pub fn hello() {
    println!("Hello, world!");
}
