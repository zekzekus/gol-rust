use std::env;

use bracket_lib::prelude::*;

use bedelli::Rule;
use bedelli::{Seeder, World};

struct GameState {
    world: World,
}

impl GameState {
    fn new(width: i32, height: i32, seeder: &Seeder, rule: Rule) -> Self {
        let world = World::new(width, height, seeder, rule);
        GameState { world }
    }
}

impl bracket_lib::prelude::GameState for GameState {
    fn tick(&mut self, ctx: &mut BTerm) {
        match ctx.key {
            Some(VirtualKeyCode::Escape) => {
                ctx.quit();
            }
            _ => {}
        }

        self.world = self.world.next();
        self.world.render(ctx);
    }
}

fn main() -> BError {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        panic!("at least four arguments required! width, height, rule and initial board type");
    }
    let width = args[1].parse::<i32>().unwrap();
    let height = args[2].parse::<i32>().unwrap();
    let rulestr = &args[3];
    let seeder_type = args[4].parse::<u32>().unwrap();

    let rule = Rule::new(&rulestr);
    let seeder = Seeder::new(seeder_type);
    let game_state = GameState::new(width, height, &seeder, rule);

    let context = BTermBuilder::simple(width, height)
        .unwrap()
        .with_title("Conway Game of Life")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, game_state)
}
