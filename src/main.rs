mod lib;
use lib::*;

fn main() {
    let a = Point { x: 0, y: 0 };
    let b = Point { x: 4, y: 4 };
    let mut state = PathBuilder::new(5, 5, a, b);
    let c = Point { x: 1, y: 1 };
    state.path.push(c);
}
