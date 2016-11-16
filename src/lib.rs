extern crate rand;
use self::rand::Rng;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct PathBuilder {
    pub width: i32,
    pub heigth: i32,
    pub minimal_path_len: i32,
    pub start: Point,
    pub end: Point,
    pub blacklist: Vec<Point>, // list of locations that lead to dead end
    pub path: Vec<Point>,
    pub return_len: usize, // number of steps to retreat when dead-end
}

impl PathBuilder {
    pub fn new(width: i32,
               heigth: i32,
               minimal_path_len: i32,
               start: Point,
               end: Point)
               -> PathBuilder {  
        let mut b = PathBuilder {
            width: width,
            heigth: heigth,
            minimal_path_len: minimal_path_len,
            start: start,
            end: end,
            blacklist: vec![],
            path: vec![],
            return_len: 0,
        };
        if !b.is_valid(b.start) {
            panic!("Start point is invalid")
        } 
        if !b.is_valid(b.end) {
            panic!("End point is invalid")
        }
        b.path.push(b.start);
        b
    }

    pub fn is_valid(&self, point: Point) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.heigth
    }

    pub fn is_free(&self, point: Point) -> bool {
        // while path is too small, path.tend is not free
        if self.path.len() < self.minimal_path_len as usize {
            !self.blacklist.contains(&point) && !self.path.contains(&point) &&
            self.is_valid(point) && point != self.end
        } else {
            !self.blacklist.contains(&point) && !self.path.contains(&point) && self.is_valid(point)
        }

    }
    // point which is free and has 3 free neighbours (the 4th is the current path end)
    pub fn is_walkable(&self, point: Point) -> bool {
        // path.end is always walkable when path is long enough
        if point == self.end && self.path.len() > self.minimal_path_len as usize {
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
        if self.path.len() >= self.minimal_path_len as usize &&
           is_neighbours(*self.path.last().expect("walk: self.path is empty 1"),
                         self.end) {
            return false;
        }
        // self.path is never empty, new() pushes start into it

        println!("self.path: {:?}", self.path);

        let walkable_neighbours =
            self.get_walkable_neighbours(*self.path.last().expect("walk: self.path is empty 2"));

        println!("walkable neighbours: {:?}", walkable_neighbours);

        if walkable_neighbours.len() > 0 {
            if walkable_neighbours.len() == 1 {
                self.return_len += 1;
            } else {
                self.return_len = 1;
            }
            // choose one at random and advance
            let next_step = rand::thread_rng().choose(walkable_neighbours.as_slice());
            self.path.push(*next_step.unwrap());
        } else {
            // nowhere yt
            self.return_len += 1;
            for _ in 0..self.return_len {
                let r = self.path.pop().unwrap();
                self.blacklist.push(r);
            }
            self.return_len = 1;
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