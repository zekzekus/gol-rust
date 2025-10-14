use bracket_lib::prelude::*;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::*;
use std::collections::HashMap;

use crate::components::*;
use crate::resources::*;

#[system]
#[read_component(Position)]
#[read_component(Cell)]
pub fn neighbor_counting(world: &mut SubWorld, cmd: &mut CommandBuffer, #[resource] dimensions: &Dimensions, #[resource] rule: &RuleFn) {
    let width = dimensions.width;
    let height = dimensions.height;

    let mut states = HashMap::new();
    let mut query = <(&Position, &Cell)>::query();
    for (pos, cell) in query.iter(world) {
        states.insert((pos.x, pos.y), cell.alive);
    }

    let mut query = <(Entity, &Position, &Cell)>::query();
    for (entity, pos, cell) in query.iter(world) {
        let neighbours = get_neighbours((pos.x, pos.y), width, height);
        let alive_neighbours = neighbours
            .iter()
            .filter(|&&pos| states.get(&pos).copied().unwrap_or(false))
            .count() as i32;
        
        let new_state = rule(cell.alive, alive_neighbours);
        cmd.add_component(*entity, NextCell { alive: new_state });
    }
}

#[system]
#[read_component(NextCell)]
#[write_component(Cell)]
pub fn state_update(world: &mut SubWorld) {
    let mut query = <(&NextCell, &mut Cell)>::query();
    for (next_cell, cell) in query.iter_mut(world) {
        cell.alive = next_cell.alive;
    }
}



pub fn render_system(world: &World, ctx: &mut BTerm) {
    ctx.cls();
    
    let mut query_with_color = <(&Position, &Cell, &CellColor)>::query();
    for (pos, cell, color) in query_with_color.iter(world) {
        let disp = if cell.alive { 'O' } else { ' ' };
        ctx.set(
            pos.x,
            pos.y,
            RGB::from_u8(color.r, color.g, color.b),
            RGB::named(BLACK),
            to_cp437(disp),
        );
    }
    
    let mut query_without_color = <(&Position, &Cell)>::query().filter(!component::<CellColor>());
    for (pos, cell) in query_without_color.iter(world) {
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
