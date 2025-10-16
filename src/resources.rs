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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conway_rule_live_cell_survives_with_2_neighbors() {
        assert_eq!(conway_rule(true, 2), true);
    }

    #[test]
    fn test_conway_rule_live_cell_survives_with_3_neighbors() {
        assert_eq!(conway_rule(true, 3), true);
    }

    #[test]
    fn test_conway_rule_live_cell_dies_with_fewer_than_2_neighbors() {
        assert_eq!(conway_rule(true, 0), false);
        assert_eq!(conway_rule(true, 1), false);
    }

    #[test]
    fn test_conway_rule_live_cell_dies_with_more_than_3_neighbors() {
        assert_eq!(conway_rule(true, 4), false);
        assert_eq!(conway_rule(true, 5), false);
        assert_eq!(conway_rule(true, 8), false);
    }

    #[test]
    fn test_conway_rule_dead_cell_becomes_alive_with_3_neighbors() {
        assert_eq!(conway_rule(false, 3), true);
    }

    #[test]
    fn test_conway_rule_dead_cell_stays_dead() {
        assert_eq!(conway_rule(false, 0), false);
        assert_eq!(conway_rule(false, 1), false);
        assert_eq!(conway_rule(false, 2), false);
        assert_eq!(conway_rule(false, 4), false);
    }

    #[test]
    fn test_highlife_rule_birth_with_3_neighbors() {
        assert_eq!(highlife_rule(false, 3), true);
    }

    #[test]
    fn test_highlife_rule_birth_with_6_neighbors() {
        assert_eq!(highlife_rule(false, 6), true);
    }

    #[test]
    fn test_highlife_rule_survive_with_2_neighbors() {
        assert_eq!(highlife_rule(true, 2), true);
    }

    #[test]
    fn test_highlife_rule_survive_with_3_neighbors() {
        assert_eq!(highlife_rule(true, 3), true);
    }

    #[test]
    fn test_highlife_rule_death() {
        assert_eq!(highlife_rule(true, 0), false);
        assert_eq!(highlife_rule(true, 1), false);
        assert_eq!(highlife_rule(true, 4), false);
        assert_eq!(highlife_rule(false, 2), false);
        assert_eq!(highlife_rule(false, 5), false);
    }

    #[test]
    fn test_day_and_night_rule_birth() {
        assert_eq!(day_and_night_rule(false, 3), true);
        assert_eq!(day_and_night_rule(false, 6), true);
        assert_eq!(day_and_night_rule(false, 7), true);
        assert_eq!(day_and_night_rule(false, 8), true);
    }

    #[test]
    fn test_day_and_night_rule_survive() {
        assert_eq!(day_and_night_rule(true, 3), true);
        assert_eq!(day_and_night_rule(true, 4), true);
        assert_eq!(day_and_night_rule(true, 6), true);
        assert_eq!(day_and_night_rule(true, 7), true);
        assert_eq!(day_and_night_rule(true, 8), true);
    }

    #[test]
    fn test_day_and_night_rule_death() {
        assert_eq!(day_and_night_rule(true, 0), false);
        assert_eq!(day_and_night_rule(true, 1), false);
        assert_eq!(day_and_night_rule(true, 2), false);
        assert_eq!(day_and_night_rule(true, 5), false);
        assert_eq!(day_and_night_rule(false, 0), false);
        assert_eq!(day_and_night_rule(false, 2), false);
        assert_eq!(day_and_night_rule(false, 5), false);
    }

    #[test]
    fn test_parse_rule_conway() {
        let rule = parse_rule("b3s23");
        assert_eq!(rule(true, 2), true);
        assert_eq!(rule(true, 3), true);
        assert_eq!(rule(false, 3), true);
        assert_eq!(rule(true, 4), false);
    }

    #[test]
    fn test_parse_rule_highlife() {
        let rule = parse_rule("b36s23");
        assert_eq!(rule(false, 6), true);
        assert_eq!(rule(false, 3), true);
        assert_eq!(rule(true, 2), true);
    }

    #[test]
    fn test_parse_rule_day_and_night() {
        let rule = parse_rule("b3678s34678");
        assert_eq!(rule(false, 8), true);
        assert_eq!(rule(true, 4), true);
    }

    #[test]
    fn test_parse_rule_invalid_defaults_to_conway() {
        let rule = parse_rule("b45s67");
        assert_eq!(rule(true, 2), true);
        assert_eq!(rule(false, 3), true);
    }
}
