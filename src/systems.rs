use bracket_lib::prelude::*;
use legion::*;
use std::collections::HashMap;

use crate::components::*;

pub fn calculate_next_generation(world: &mut World, resources: &mut Resources) {
    let dimensions = resources.get::<Dimensions>().unwrap();
    let rule = resources.get::<Rule>().unwrap();
    let width = dimensions.width;
    let height = dimensions.height;

    let mut states = HashMap::new();
    
    let mut query = <(&Position, &Cell)>::query();
    for (pos, cell) in query.iter(world) {
        states.insert((pos.x, pos.y), cell.alive);
    }

    let mut next_states = HashMap::new();
    for ((x, y), alive) in &states {
        let neighbours = get_neighbours((*x, *y), width, height);
        let alive_neighbours = neighbours
            .iter()
            .filter(|&&pos| states.get(&pos).copied().unwrap_or(false))
            .count() as i32;
        
        let new_state = rule.check(*alive, alive_neighbours);
        next_states.insert((*x, *y), new_state);
    }

    let mut query = <(&Position, &mut Cell)>::query();
    for (pos, cell) in query.iter_mut(world) {
        if let Some(&new_state) = next_states.get(&(pos.x, pos.y)) {
            cell.alive = new_state;
        }
    }
}

pub fn render_system(world: &World, ctx: &mut BTerm) {
    ctx.cls();
    let mut query = <(&Position, &Cell)>::query();
    for (pos, cell) in query.iter(world) {
        let disp = if cell.alive { 'O' } else { ' ' };
        ctx.set(
            pos.x,
            pos.y,
            RGB::named(WHITE),
            RGB::named(BLACK),
            to_cp437(disp),
        );
    }
}

fn get_neighbours(point: (i32, i32), max_width: i32, max_height: i32) -> Vec<(i32, i32)> {
    let mut points = Vec::new();
    let range = [-1, 0, 1];
    for row in &range {
        for col in &range {
            if !(*row == *col && *row == 0) {
                let nx = *row + point.0;
                let ny = *col + point.1;
                if nx >= 0 && nx < max_width && ny >= 0 && ny < max_height {
                    points.push((nx, ny));
                }
            }
        }
    }
    points
}
