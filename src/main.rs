mod lib;
use lib::*;

extern crate tcod;
use self::tcod::console::*;
use self::tcod::colors;

fn main() {
    let a = Point { x: 1, y: 1 };
    let b = Point { x: 8, y: 8 };
    let mut state = PathBuilder::new(10, 10, 5, a, b);
    for i in 0..100 {
        let x = state.walk();
        
        println!{"cycle: {}", i};
        println!{"path: {:?}", state.path};
        println!{"blacklist: {:?}", state.blacklist};
        
        if !x {
            break;
        }
    }
    // let p = Point{x: 1, y: 8};
    // state.path.push(p);
    // state.path.push(Point{x: 5, y: 6});
    // state.path.push(Point{x: 5, y: 4});
    // state.path.push(Point{x: 4, y: 5});
    // println!{"f.n.: {:?}", state.get_free_neighbours(p)};
    // println!{"w.n.: {:?}", state.get_walkable_neighbours(p)};
    
}
