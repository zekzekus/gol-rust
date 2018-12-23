use std::env;
use std::process::exit;

use tcod::input;
use tcod::input::Event;
use tcod::input::KeyCode;
use tcod::system;
use tcod::RootConsole;

use bedelli::Rule;
use bedelli::{Seeder, World};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        panic!("at least four arguments required! width, height, rule and initial board type");
    }
    let width = args[1].parse::<i32>().unwrap();
    let height = args[2].parse::<i32>().unwrap();
    let rulestr = &args[3];
    let seeder = args[4].parse::<u32>().unwrap();

    let mut con = RootConsole::initializer()
        .size(width, height)
        .title("Conway Game of Life")
        .init();

    let rule = Rule::new(&rulestr);
    let mut world: World<'_> = World::new(width, height, &Seeder::new(seeder), &rule);

    system::set_fps(30);
    world.render(&mut con);
    while !con.window_closed() {
        if let Some(input) = user_input() {
            match input {
                Input::Exit => {
                    println!("User exit");
                    exit(0);
                }
            }
        }

        world = world.next();
        world.render(&mut con);
    }
}
enum Input {
    Exit,
}

fn user_input() -> Option<Input> {
    let flags = input::KEY;
    match input::check_for_event(flags).map(|(_, e)| e) {
        Some(Event::Key(s)) => match s.code {
            KeyCode::Escape => Some(Input::Exit),
            _ => None,
        },
        _ => None,
    }
}
