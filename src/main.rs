#![feature(plugin)]
#![plugin(clippy)]
extern crate bedelliv2;

use std::env;
use std::thread::sleep_ms;

use bedelliv2::{World, Seeder};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("at least two arguments required! width and height");
    }
    let width = args[1].parse::<i32>().unwrap();
    let height = args[2].parse::<i32>().unwrap();

    let mut world: World = World::new(width, height, Seeder::Random);
    println!("initial world");
    world.print();
    for _ in 1.. {
        world = world.next();
        world.print();
        sleep_ms(50);
    }
}
