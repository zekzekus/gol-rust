use std::env;

use bracket_lib::prelude::*;
use legion::*;

use bedelli::components::*;
use bedelli::resources::*;
use bedelli::systems::*;
use bedelli::Seeder;

use bedelli::resources::{parse_rule, RuleFn, InputState, PositionIndex};

struct GameState {
    ecs: World,
    resources: Resources,
    schedule: Schedule,
}

impl GameState {
    fn new(width: i32, height: i32, seeder: &Seeder, rule: RuleFn) -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();

        let grid = seeder.seed(width, height);

        let mut position_index = std::collections::HashMap::new();
        for ((x, y), state) in grid {
            let entity = ecs.push((
                Position { x, y },
                Cell { alive: state == 1 },
                Age::default(),
                CellColor::default(),
            ));
            position_index.insert((x, y), entity);
        }

        resources.insert(Dimensions { width, height });
        resources.insert(rule);
        resources.insert(PositionIndex(position_index));
        resources.insert(InputState::default());

        let schedule = Schedule::builder()
            .add_system(mouse_toggle_system())
            .add_system(neighbor_counting_system())
            .add_system(state_update_system())
            .add_system(cleanup_next_system())
            .add_system(age_update_system())
            .add_system(color_update_system())
            .build();

        GameState {
            ecs,
            resources,
            schedule,
        }
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

        let mouse_pos = ctx.mouse_point();
        let mouse_left = ctx.left_click;

        if let Some(mut input_state) = self.resources.get_mut::<InputState>() {
            input_state.mouse_pos = (mouse_pos.x, mouse_pos.y);
            input_state.mouse_left = mouse_left;
        }

        self.schedule.execute(&mut self.ecs, &mut self.resources);
        render_system(&self.ecs, ctx);
    }
}

fn main() -> BError {
    // Force X11 backend on Linux to avoid Wayland compositor performance issues
    #[cfg(target_os = "linux")]
    env::remove_var("WAYLAND_DISPLAY");

    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        panic!("at least four arguments required! width, height, rule and initial board type");
    }
    let width = args[1].parse::<i32>().unwrap();
    let height = args[2].parse::<i32>().unwrap();
    let rulestr = &args[3];
    let seeder_type = args[4].parse::<u32>().unwrap();

    let rule = parse_rule(&rulestr);
    let seeder = Seeder::new(seeder_type);
    let game_state = GameState::new(width, height, &seeder, rule);

    let context = BTermBuilder::simple(width, height)
        .unwrap()
        .with_title("Conway Game of Life")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, game_state)
}
