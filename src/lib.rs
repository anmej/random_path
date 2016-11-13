#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct PathBuilder {
    pub width: i32,
    pub heigth: i32,
    pub start: Point,
    pub end: Point,
    pub blacklist: Vec<Point>, // list of locations that lead to dead end
    pub path: Vec<Point>,
}

impl PathBuilder {
    pub fn new(width: i32, heigth: i32, start: Point, end: Point) -> PathBuilder {
        PathBuilder {
            width: width,
            heigth: heigth,
            start: start,
            end: end,
            blacklist: vec![],
            path: vec![],
        }
    }

    pub fn is_valid(&self, point: Point) -> bool {
        point.x > 0 && point.x < self.width && point.y > 0 && point.y < self.heigth
    }

    pub fn is_free(&self, point: Point) -> bool {
        !self.blacklist.contains(&point) && !self.path.contains(&point) && self.is_valid(point)
    }

    pub fn get_free_neighbours(&self, point: Point) -> Vec<Point> {
        let mut free_neighbours = vec![];
        for (dx, dy) in [1, -1, 0, 0].iter().zip([0, 0, 1, -1].iter()) {
            let neighbour = Point {
                x: point.x + dx,
                y: point.y + dy,
            };
            if self.is_free(neighbour) {
                free_neighbours.push(neighbour)
            }
        }
        free_neighbours
    }
}

pub fn manhattan_distance(a: Point, b: Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()

}

pub fn is_neighbours(a: Point, b: Point) -> bool {
    manhattan_distance(a, b) < 3
}