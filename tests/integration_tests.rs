use bedelli::components::*;
use bedelli::resources::*;
use bedelli::Seeder;
use legion::*;
use std::collections::HashMap;

fn create_world_with_pattern(pattern: Vec<(i32, i32)>, width: i32, height: i32) -> (World, Resources) {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut position_index = HashMap::new();

    for x in 0..width {
        for y in 0..height {
            let alive = pattern.contains(&(x, y));
            let entity = world.push((
                Position { x, y },
                Cell { alive },
                Age::default(),
                CellColor::default(),
            ));
            position_index.insert((x, y), entity);
        }
    }

    resources.insert(Dimensions { width, height });
    resources.insert(conway_rule as RuleFn);
    resources.insert(PositionIndex(position_index));

    (world, resources)
}

fn get_alive_positions(world: &World) -> Vec<(i32, i32)> {
    <(&Position, &Cell)>::query()
        .iter(world)
        .filter_map(|(pos, cell)| {
            if cell.alive {
                Some((pos.x, pos.y))
            } else {
                None
            }
        })
        .collect()
}

fn simulate_step(world: &mut World, resources: &mut Resources) {
    use std::collections::HashSet;

    let dimensions = resources.get::<Dimensions>().unwrap();
    let width = dimensions.width;
    let height = dimensions.height;
    let rule = *resources.get::<RuleFn>().unwrap();

    let alive: HashSet<(i32, i32)> = get_alive_positions(world)
        .into_iter()
        .collect();

    let offsets: [(i32, i32); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    let mut updates = Vec::new();
    let mut query = <(Entity, &Position, &Cell)>::query();
    for (entity, pos, cell) in query.iter(world) {
        let mut alive_neighbours = 0;
        for (dx, dy) in offsets {
            let nx = pos.x + dx;
            let ny = pos.y + dy;
            if nx >= 0 && nx < width && ny >= 0 && ny < height && alive.contains(&(nx, ny)) {
                alive_neighbours += 1;
            }
        }
        
        let new_state = rule(cell.alive, alive_neighbours);
        updates.push((*entity, new_state));
    }

    for (entity, new_state) in updates {
        if let Some(mut entry) = world.entry(entity) {
            if let Ok(cell) = entry.get_component_mut::<Cell>() {
                cell.alive = new_state;
            }
        }
    }
}

#[test]
fn test_block_pattern_stays_stable() {
    let block = vec![(1, 1), (1, 2), (2, 1), (2, 2)];
    let (mut world, mut resources) = create_world_with_pattern(block.clone(), 5, 5);

    simulate_step(&mut world, &mut resources);
    
    let alive = get_alive_positions(&world);
    assert_eq!(alive.len(), 4);
    for pos in &block {
        assert!(alive.contains(pos));
    }
}

#[test]
fn test_blinker_oscillates() {
    let blinker_horizontal = vec![(2, 1), (2, 2), (2, 3)];
    let blinker_vertical = vec![(1, 2), (2, 2), (3, 2)];
    
    let (mut world, mut resources) = create_world_with_pattern(blinker_horizontal.clone(), 5, 5);

    simulate_step(&mut world, &mut resources);
    let alive = get_alive_positions(&world);
    
    assert_eq!(alive.len(), 3);
    for pos in &blinker_vertical {
        assert!(alive.contains(pos));
    }

    simulate_step(&mut world, &mut resources);
    let alive = get_alive_positions(&world);
    
    assert_eq!(alive.len(), 3);
    for pos in &blinker_horizontal {
        assert!(alive.contains(pos));
    }
}

#[test]
fn test_glider_moves() {
    let glider_gen0 = vec![(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)];
    let (mut world, mut resources) = create_world_with_pattern(glider_gen0, 10, 10);

    let initial_alive = get_alive_positions(&world);
    
    for _ in 0..4 {
        simulate_step(&mut world, &mut resources);
    }
    
    let final_alive = get_alive_positions(&world);
    
    assert_eq!(final_alive.len(), 5);
    
    let has_moved = initial_alive.iter().any(|pos| !final_alive.contains(pos));
    assert!(has_moved);
}

#[test]
fn test_beehive_stays_stable() {
    let beehive = vec![(2, 1), (1, 2), (3, 2), (1, 3), (3, 3), (2, 4)];
    let (mut world, mut resources) = create_world_with_pattern(beehive.clone(), 6, 6);

    simulate_step(&mut world, &mut resources);
    
    let alive = get_alive_positions(&world);
    assert_eq!(alive.len(), 6);
    for pos in &beehive {
        assert!(alive.contains(pos));
    }
}

#[test]
fn test_empty_grid_stays_empty() {
    let empty = vec![];
    let (mut world, mut resources) = create_world_with_pattern(empty, 5, 5);

    simulate_step(&mut world, &mut resources);
    
    let alive = get_alive_positions(&world);
    assert_eq!(alive.len(), 0);
}

#[test]
fn test_single_cell_dies() {
    let single = vec![(2, 2)];
    let (mut world, mut resources) = create_world_with_pattern(single, 5, 5);

    simulate_step(&mut world, &mut resources);
    
    let alive = get_alive_positions(&world);
    assert_eq!(alive.len(), 0);
}

#[test]
fn test_glider_seeder_creates_valid_pattern() {
    let seeder = Seeder::Glider;
    let grid = seeder.seed(10, 10);
    
    let alive_count = grid.values().filter(|&&v| v == 1).count();
    assert_eq!(alive_count, 5);
    
    assert_eq!(grid.get(&(1, 2)), Some(&1));
    assert_eq!(grid.get(&(2, 3)), Some(&1));
    assert_eq!(grid.get(&(3, 1)), Some(&1));
    assert_eq!(grid.get(&(3, 2)), Some(&1));
    assert_eq!(grid.get(&(3, 3)), Some(&1));
}

#[test]
fn test_multiple_simulation_steps() {
    let blinker = vec![(2, 1), (2, 2), (2, 3)];
    let (mut world, mut resources) = create_world_with_pattern(blinker.clone(), 5, 5);

    for _ in 0..10 {
        simulate_step(&mut world, &mut resources);
    }
    
    let alive = get_alive_positions(&world);
    assert_eq!(alive.len(), 3);
}
