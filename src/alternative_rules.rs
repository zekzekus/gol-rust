use crate::resources::CellularRule;

#[derive(Clone, Debug)]
pub struct HighLifeRule;

impl CellularRule for HighLifeRule {
    fn check(&self, curr_state: bool, neighbours_alive: i32) -> bool {
        match curr_state {
            true if [2, 3].contains(&neighbours_alive) => true,
            false if [3, 6].contains(&neighbours_alive) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DayAndNightRule;

impl CellularRule for DayAndNightRule {
    fn check(&self, curr_state: bool, neighbours_alive: i32) -> bool {
        match curr_state {
            true if [3, 4, 6, 7, 8].contains(&neighbours_alive) => true,
            false if [3, 6, 7, 8].contains(&neighbours_alive) => true,
            _ => false,
        }
    }
}
