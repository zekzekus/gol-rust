#![feature(plugin)]
#![plugin(clippy)]
extern crate bedelli;
extern crate tcod;

use std::env;
use std::process::exit;

use tcod::RootConsole;
use tcod::input;
use tcod::input::Event;
use tcod::input::KeyCode;
use tcod::system;


use bedelli::{World, Seeder};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("at least two arguments required! width and height");
    }
    let width = args[1].parse::<i32>().unwrap();
    let height = args[2].parse::<i32>().unwrap();

    let mut con = RootConsole::initializer()
        .size(width, height)
        .title("Conway Game of Life")
        .init();

    let mut world: World = World::new(width, height, Seeder::Random);

    system::set_fps(30);
    world.render(&mut con);
    while !con.window_closed() {
        if let Some(input) = user_input() {
            match input {
                Input::Exit => {
                    println!("User exit");
                    exit(0);
                },
            }
        }

        world = world.next();
        world.render(&mut con);
    }
}
enum Input { Exit, }

fn user_input() -> Option<Input> {
    let flags = input::KEY;
    match input::check_for_event(flags).map(|(_, e)| e) {
        Some(Event::Key(s)) => {
            match s.code {
                KeyCode::Escape => Some(Input::Exit),
                _ => None
            }
        },
        _ => None
    }
}
