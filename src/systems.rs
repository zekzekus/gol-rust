use bracket_lib::prelude::*;
use legion::*;
use std::collections::HashMap;

use crate::components::*;
use crate::resources::*;

pub fn neighbor_counting_system(world: &mut World, resources: &mut Resources) {
    let dimensions = resources.get::<Dimensions>().unwrap();
    let rule = resources.get::<Rule>().unwrap();
    let width = dimensions.width;
    let height = dimensions.height;

    let mut states = HashMap::new();
    let mut query = <(&Position, &Cell)>::query();
    for (pos, cell) in query.iter(world) {
        states.insert((pos.x, pos.y), cell.alive);
    }

    let mut next_cells = Vec::new();
    let mut query = <(Entity, &Position, &Cell)>::query();
    for (entity, pos, cell) in query.iter(world) {
        let neighbours = get_neighbours((pos.x, pos.y), width, height);
        let alive_neighbours = neighbours
            .iter()
            .filter(|&&pos| states.get(&pos).copied().unwrap_or(false))
            .count() as i32;
        
        let new_state = rule.check(cell.alive, alive_neighbours);
        next_cells.push((*entity, NextCell { alive: new_state }));
    }

    for (entity, next_cell) in next_cells {
        world.entry(entity).unwrap().add_component(next_cell);
    }
}

pub fn state_update_system(world: &mut World) {
    let mut query = <(&NextCell, &mut Cell)>::query();
    for (next_cell, cell) in query.iter_mut(world) {
        cell.alive = next_cell.alive;
    }
}

pub fn calculate_next_generation(world: &mut World, resources: &mut Resources) {
    neighbor_counting_system(world, resources);
    state_update_system(world);
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
