use bracket_lib::prelude::*;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::*;
use std::collections::HashSet;

use crate::components::*;
use crate::resources::*;

const OFFSETS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),           (0, 1),
    (1, -1),  (1, 0),  (1, 1),
];

#[system]
#[read_component(Position)]
#[read_component(Cell)]
pub fn neighbor_counting(world: &mut SubWorld, cmd: &mut CommandBuffer, #[resource] dimensions: &Dimensions, #[resource] rule: &RuleFn) {
    let width = dimensions.width;
    let height = dimensions.height;

    let alive: HashSet<(i32, i32)> = <(&Position, &Cell)>::query()
        .iter(world)
        .filter_map(|(pos, cell)| {
            if cell.alive {
                Some((pos.x, pos.y))
            } else {
                None
            }
        })
        .collect();

    let mut query = <(Entity, &Position, &Cell)>::query();
    for (entity, pos, cell) in query.iter(world) {
        let mut alive_neighbours = 0;
        for (dx, dy) in OFFSETS {
            let nx = pos.x + dx;
            let ny = pos.y + dy;
            if nx >= 0 && nx < width && ny >= 0 && ny < height && alive.contains(&(nx, ny)) {
                alive_neighbours += 1;
            }
        }
        
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

#[system]
#[read_component(Cell)]
#[write_component(Age)]
pub fn age_update(world: &mut SubWorld) {
    let mut query = <(&Cell, &mut Age)>::query();
    for (cell, age) in query.iter_mut(world) {
        if cell.alive {
            age.value = age.value.saturating_add(1);
        } else {
            age.value = 0;
        }
    }
}

#[system]
#[read_component(Age)]
#[write_component(CellColor)]
pub fn color_update(world: &mut SubWorld) {
    let mut query = <(&Age, &mut CellColor)>::query();
    for (age, color) in query.iter_mut(world) {
        let age_capped = age.value.min(60);
        let intensity = 255 - (age_capped as f32 * 2.5) as u8;
        color.r = intensity;
        color.g = intensity.saturating_add(50);
        color.b = 255;
    }
}

#[system]
#[write_component(Cell)]
pub fn mouse_toggle(world: &mut SubWorld, #[resource] input: &InputState, #[resource] index: &PositionIndex) {
    if !input.mouse_left {
        return;
    }
    
    if let Some(&entity) = index.0.get(&input.mouse_pos) {
        if let Ok(mut entry) = world.entry_mut(entity) {
            if let Ok(cell) = entry.get_component_mut::<Cell>() {
                cell.alive = !cell.alive;
            }
        }
    }
}



pub fn render_system(world: &World, ctx: &mut BTerm) {
    ctx.cls();
    
    let mut query = <(&Position, &Cell, &CellColor)>::query();
    for (pos, cell, color) in query.iter(world) {
        let disp = if cell.alive { 'O' } else { ' ' };
        ctx.set(
            pos.x,
            pos.y,
            RGB::from_u8(color.r, color.g, color.b),
            RGB::named(BLACK),
            to_cp437(disp),
        );
    }
}


