pub type RuleFn = fn(bool, i32) -> bool;

pub fn parse_rule(rule_str: &str) -> RuleFn {
    let parts: Vec<&str> = rule_str.split('s').collect();
    let born_part: Vec<&str> = parts[0].matches(char::is_numeric).collect();
    let stay_part: Vec<&str> = parts[1].matches(char::is_numeric).collect();

    let born_keys: Vec<i32> = born_part
        .iter()
        .map(|&each| each.parse::<i32>().unwrap())
        .collect();
    let stay_keys: Vec<i32> = stay_part
        .iter()
        .map(|&each| each.parse::<i32>().unwrap())
        .collect();
    
    make_rule(born_keys, stay_keys)
}

fn make_rule(borns: Vec<i32>, stays: Vec<i32>) -> RuleFn {
    if borns == vec![3] && stays == vec![2, 3] {
        conway_rule
    } else if borns == vec![3, 6] && stays == vec![2, 3] {
        highlife_rule
    } else if borns == vec![3, 6, 7, 8] && stays == vec![3, 4, 6, 7, 8] {
        day_and_night_rule
    } else {
        conway_rule
    }
}

pub fn conway_rule(curr_state: bool, neighbours_alive: i32) -> bool {
    match curr_state {
        true if [2, 3].contains(&neighbours_alive) => true,
        false if neighbours_alive == 3 => true,
        _ => false,
    }
}

pub fn highlife_rule(curr_state: bool, neighbours_alive: i32) -> bool {
    match curr_state {
        true if [2, 3].contains(&neighbours_alive) => true,
        false if [3, 6].contains(&neighbours_alive) => true,
        _ => false,
    }
}

pub fn day_and_night_rule(curr_state: bool, neighbours_alive: i32) -> bool {
    match curr_state {
        true if [3, 4, 6, 7, 8].contains(&neighbours_alive) => true,
        false if [3, 6, 7, 8].contains(&neighbours_alive) => true,
        _ => false,
    }
}

pub struct Dimensions {
    pub width: i32,
    pub height: i32,
}

use std::collections::HashMap;
use legion::Entity;

pub struct InputState {
    pub mouse_pos: (i32, i32),
    pub mouse_left: bool,
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            mouse_pos: (0, 0),
            mouse_left: false,
        }
    }
}

pub struct PositionIndex(pub HashMap<(i32, i32), Entity>);
