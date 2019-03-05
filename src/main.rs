extern crate dogeylvania;
extern crate rand;
extern crate tcod;

use dogeylvania::dogemaths::*;
use dogeylvania::maps::*;
use tcod::colors::{self, Color};
use tcod::console::*;
use tcod::map::{FovAlgorithm, Map as FovMap};

fn main() {
    let mut map = Map::new_default(20, 20, 0);
    for y in 0..map.height() {
        for x in 0..map.width() {
            print!("{}", map.get(x, y));
        }
        println!("");
    }
    println!("{}", clamp(1.5, 0.5, 1.4));
}
