#[derive(Clone, Debug)]
pub struct Rule {
    pub borns: Vec<i32>,
    pub stays: Vec<i32>,
}

impl Rule {
    pub fn new(rule: &str) -> Self {
        let parts: Vec<&str> = rule.split('s').collect();
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
        Rule {
            borns: born_keys,
            stays: stay_keys,
        }
    }

    pub fn check(&self, curr_state: bool, neighbours_alive: i32) -> bool {
        match curr_state {
            true if self.stays.contains(&neighbours_alive) => true,
            false if self.borns.contains(&neighbours_alive) => true,
            _ => false,
        }
    }
}

pub struct Dimensions {
    pub width: i32,
    pub height: i32,
}
