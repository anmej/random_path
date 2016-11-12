#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn manhattan_distance(a: Point, b: Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()

}

pub fn is_neighbours(a: Point, b: Point) -> bool {
    manhattan_distance(a, b) < 3
}

pub fn hello() {
    println!("Hello, world!");
}
