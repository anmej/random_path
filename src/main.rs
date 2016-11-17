mod lib;
use lib::*;

extern crate tcod;
use self::tcod::console::*;
use self::tcod::colors;

fn main() {
    let a = Point { x: 0, y: 19 };
    let b = Point { x: 99, y: 0 };
    let mut state = PathBuilder::new(100, 20, 10, a, b);
    let mut c = 0;

    for i in 0..100000 {
        let x = state.walk();
        c = i;
        // println!{"cycle: {}", i};
        // println!{"path: {:?}", state.path};
        // println!{"blacklist: {:?}", state.blacklist};
        // println!{"{}", state};
        if !x {
            break;
        }
    }
    println!{"cycle: {}", c};
    println!{"{}", state};
    // let p = Point{x: 1, y: 8};
    // state.path.push(p);
    // state.path.push(Point{x: 5, y: 6});
    // state.path.push(Point{x: 5, y: 4});
    // state.path.push(Point{x: 4, y: 5});
    // println!{"f.n.: {:?}", state.get_free_neighbours(p)};
    // println!{"w.n.: {:?}", state.get_walkable_neighbours(p)};

}
