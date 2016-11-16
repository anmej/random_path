mod lib;
use lib::*;

fn main() {
    let a = Point { x: 0, y: 0 };
    let b = Point { x: 0, y: 0 };
    let mut state = PathBuilder::new(5, 5, 5, a, b);
    // for i in 0..100 {
    //     let x = state.walk();
        
    //     println!{"cycle: {}", i};
    //     println!{"path: {:?}", state.path};
    //     println!{"blacklist: {:?}", state.blacklist};
        
    //     if !x {
    //         break;
    //     }
    // }
    let p = Point{x: 1, y: 0};
    println!{"f.n.: {:?}", state.get_free_neighbours(p)};
    println!{"w.n.: {:?}", state.get_walkable_neighbours(p)};
    
}
