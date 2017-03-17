extern crate rand;
use self::rand::Rng;

// extern crate tcod;
// use self::tcod::console::*;
// use self::tcod::colors;

use std::fmt;

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

impl fmt::Display for PathBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.heigth {
            for j in 0..self.width {
                let p = Point { x: j, y: i };
                if self.start == p {
                    write!(f, "S").ok();
                } else if self.end == p {
                    write!(f, "E").ok();
                } else if self.path.contains(&p) {
                    write!(f, "█").ok();
                } else if self.blacklist.contains(&p) {
                    write!(f, "x").ok();
                } else {
                    write!(f, "░").ok();
                }
            }
            write!(f, "\n").ok();
        }
        write!(f, "____________________________")
    }
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
        // while path is too small, path.end is not free
        if self.path.len() < self.minimal_path_len as usize {
            !self.blacklist.contains(&point) && !self.path.contains(&point) && point != self.end
        } else {
            !self.blacklist.contains(&point) && !self.path.contains(&point)
        }

    }
    // point which is free and has 3 free neighbours (the 4th is the current path end)
    pub fn is_walkable(&self, point: Point) -> bool {
        // path.end is always walkable when path is long enough
        if point == self.end && self.path.len() > self.minimal_path_len as usize {
            true
        } else {
            self.is_free(point) && self.is_valid(point) &&
            self.get_free_neighbours(point).len() >= 3
        }
    }

    pub fn get_free_neighbours(&self, point: Point) -> Vec<Point> {
        let mut free_neighbours = Vec::with_capacity(4);
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
        let mut walkable_neighbours = Vec::with_capacity(4);
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
        // returns false when the path is complete

        // end condition
        if self.path.len() >= self.minimal_path_len as usize &&
           *self.path.last().expect("walk: self.path is empty 1") == self.end {
            return false;
        }
        // nowhere to go, no path - clear the board, start again
        if self.path.len() <= 1 {
            self.path.push(self.start);
            self.blacklist.clear();
        }

        let walkable_neighbours =
            self.get_walkable_neighbours(*self.path.last().expect("walk: self.path is empty 2"));

        if walkable_neighbours.len() > 0 {
            // calculating the length of deterministic path
            if walkable_neighbours.len() == 1 {
                self.return_len += 1;
            } else {
                self.return_len = 1;
            }
            // choose one at random and advance
            let next_step = rand::thread_rng().choose(walkable_neighbours.as_slice());
            self.path.push(*next_step.unwrap());
        } else {
            // dead end: retreating and blacklisting
            self.return_len += 1;
            if self.path.len() < self.return_len {
                panic!("self.path.len() < self.return_len");
            }
            for _ in 0..self.return_len {
                let r = self.path.pop().expect("nothing to pop from self.path");
                self.blacklist.push(r);
            }
            self.return_len = 1;
        }
        true
    }
    pub fn tcod_render() {}
}