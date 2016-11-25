mod lib;
use lib::*;

// extern crate tcod;
// use self::tcod::console::*;
// use self::tcod::colors;
use std::env;



fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        // no arguments passed
        7 => {
            println!("7 arguments, including 0 ({})", args[0]);
        }
        _ => {
            println!("Incorrect number of arguments");
            panic!();
        }
    }
    let mut arg_array = [0i32; 6];
    for (i, arg) in args.into_iter().skip(1).enumerate() {
        if let Ok(x) = arg.parse() {
            arg_array[i] = x;
        } else {
            panic!("Argument is not i32");
        }
    }
    let a = Point { x: arg_array[2], y: arg_array[3] };
    let b = Point { x: arg_array[4], y: arg_array[5] };
    let mut state = PathBuilder::new(arg_array[0], arg_array[1], 10, a, b);
    for i in 0..100000 {
        let x = state.walk();
        // println!{"cycle: {}", i};
        // println!{"path: {:?}", state.path};
        // println!{"blacklist: {:?}", state.blacklist};
        // println!{"{}", state};
        if !x {
            println!{"cycle: {}", i};
            println!{"{}", state};
            break;
        }
    }
    // Prints each argument on a separate line
    // let p = Point{x: 1, y: 8};
    // state.path.push(p);
    // state.path.push(Point{x: 5, y: 6});
    // state.path.push(Point{x: 5, y: 4});
    // state.path.push(Point{x: 4, y: 5});
    // println!{"f.n.: {:?}", state.get_free_neighbours(p)};
    // println!{"w.n.: {:?}", state.get_walkable_neighbours(p)};

}
