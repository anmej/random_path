extern crate rand;
use self::rand::Rng;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct PathBuilder {
    pub width: i32,
    pub heigth: i32,
    pub minimal_path_length: i32,
    pub start: Point,
    pub end: Point,
    pub blacklist: Vec<Point>, // list of locations that lead to dead end
    pub path: Vec<Point>,
}

impl PathBuilder {
    pub fn new(width: i32,
               heigth: i32,
               minimal_path_length: i32,
               start: Point,
               end: Point)
               -> PathBuilder {
        let mut b = PathBuilder {
            width: width,
            heigth: heigth,
            minimal_path_length: minimal_path_length,
            start: start,
            end: end,
            blacklist: vec![],
            path: vec![],
        };
        b.path.push(b.start);
        b
    }

    pub fn is_valid(&self, point: Point) -> bool {
        point.x > 0 && point.x < self.width && point.y > 0 && point.y < self.heigth
    }

    pub fn is_free(&self, point: Point) -> bool {
        // while path is too small, path.tend is not free
        if self.path.len() < self.minimal_path_length as usize {
            !self.blacklist.contains(&point) && !self.path.contains(&point) &&
            self.is_valid(point) && point != self.end
        } else {
            !self.blacklist.contains(&point) && !self.path.contains(&point) && self.is_valid(point)
        }

    }
    // point which is free and has 3 free neighbours (the 4th is the current path end)
    pub fn is_walkable(&self, point: Point) -> bool {
        //path.end is always walkable when path is long enough
        if point == self.end && self.path.len() > self.minimal_path_length as usize {
            true
        } else {
            self.is_free(point) && self.get_free_neighbours(point).len() == 3
        }
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

    pub fn get_walkable_neighbours(&self, point: Point) -> Vec<Point> {
        let mut walkable_neighbours = vec![];
        for (dx, dy) in [1, -1, 0, 0].iter().zip([0, 0, 1, -1].iter()) {
            let neighbour = Point {
                x: point.x + dx,
                y: point.y + dy,
            };
            if self.is_walkable(neighbour) {
                walkable_neighbours.push(neighbour)
            }
        }
        walkable_neighbours
    }

    pub fn walk(&mut self) -> bool {
        // end condition
        if self.path.len() >= self.minimal_path_length as usize &&
           is_neighbours(*self.path.last().unwrap(), self.end) {
            return false;
        }
        // self.path is never empty, new() pushes start into it
        let walkable_neighbours = self.get_walkable_neighbours(*self.path.last().unwrap());
        if walkable_neighbours.len() > 0 {
            // choose one at random and advance
            let next_step = rand::thread_rng().choose(walkable_neighbours.as_slice());
            self.path.push(*next_step.unwrap());
        } else {
            // nowhere yt
            unimplemented!()
        }


        true
    }
}

pub fn manhattan_distance(a: Point, b: Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()

}

pub fn is_neighbours(a: Point, b: Point) -> bool {
    manhattan_distance(a, b) == 1
}