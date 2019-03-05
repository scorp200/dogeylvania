extern crate dogeylvania;
extern crate rand;
extern crate tcod;

use dogeylvania::dogemaths::*;
use dogeylvania::maps::*;
use tcod::colors::{self, Color};
use tcod::console::*;
use tcod::map::{FovAlgorithm, Map as FovMap};

fn main() {
    let mut arr = vec![0; 10];
    for i in 0..arr.len() {
        arr[i] = i;
    }
    for i in 0..arr.len() {
        print!("{}", arr[i]);
    }
    shufflearray(&mut arr);
    println!("");
    for i in 0..arr.len() {
        print!("{}", arr[i]);
    }
}
