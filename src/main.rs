mod lib;
use lib::*;

fn main() {
    hello();
    let a = Point {x: 1, y: 1};
    let b = Point {x: 0, y: 0};
    std::
    println!("{:?}", manhattan_distance(a, b));
    println!("{:?}", is_neighbours(a, b));
    
}
