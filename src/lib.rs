#![feature(plugin)]
#![plugin(clippy)]
extern crate tcod;

pub mod seeder;

pub use seeder::Seeder;

use std::collections::BTreeMap;
use std::collections::HashMap;

use tcod::{Console, RootConsole};
use tcod::BackgroundFlag;

// a sorted tree map to represent whole game board
// coordinated by tuples of integers. Dead or Alive status 
// 1 and 0.
pub type Grid = BTreeMap<(i32, i32), i32>;

pub struct World<'a> {
    width: i32,
    height: i32,
    pub grid: Grid,
    rule: &'a str,
}

struct Rule {
    borns: HashMap<i32, i32>,
    stays: HashMap<i32, i32>,
}

impl Rule {
    fn new(rule: &str) -> Self {
        let mut born_map: HashMap<i32, i32> = HashMap::new();
        let mut stay_map: HashMap<i32, i32> = HashMap::new();
        born_map.insert(2, 0);
        born_map.insert(3, 0);
        stay_map.insert(3, 0);
        Rule{borns: born_map, stays: stay_map}
    }
}

impl<'a> World<'a> {
    pub fn new(width: i32, height: i32, seeder: seeder::Seeder,
               rule: &'a str) -> Self {
        let grid = seeder.seed(width, height);
        World{width: width, height: height, grid: grid, rule: rule}
    }

    pub fn print(&self) {
        let mut tx = 0;
        for (key, value) in &self.grid {
            if key.0 != tx {
                tx = key.0;
                print!("\n");
            }
            if *value == 1 {
                print!("O");
            } else {
                print!("-");
            }
        }
        print!("\n");
    }

    pub fn next(&self) -> Self {
        let mut next_grid: Grid = Grid::new();
        let rule = Rule::new(self.rule);

        for (key, state) in &self.grid {
            let mut total_state: i32 = 0;
            let new_state: i32;
            for n in neighbours(*key, self.width, self.height) {
                let state = self.grid.get(&n).unwrap();
                total_state += *state;
            }
            if *state == 1 && (rule.borns.contains_key(&total_state)) {
                new_state = 1;
            } else if *state == 0 && (rule.stays.contains_key(&total_state)){
                new_state = 1;
            } else {
                new_state = 0;
            }
            next_grid.insert(*key, new_state);
        }
        World{width: self.width, height: self.height, grid: next_grid,
              rule: self.rule}
    }

    pub fn render(&self, console: &mut RootConsole) {
        console.clear();
        for (key, value) in &self.grid {
            let disp: char;
            if *value == 1 {
                disp = 'O';
            } else {
                disp = ' ';
            }
            console.put_char(key.0, key.1, disp, BackgroundFlag::Set);
        }
        console.flush();
    }

}

fn neighbours(point: (i32, i32), max_width: i32, max_height: i32) -> Vec<(i32, i32)> {
    let mut points: Vec<(i32, i32)> = Vec::new();
    let range = vec![-1i32, 0i32, 1i32];
    for row in &range {
        for col in &range {
            if !(*row == *col && *row == 0) {
                let nx = *row + point.0;
                let ny = *col + point.1;
                if 0 <= nx && nx < max_width && 0 <= ny && ny < max_height {
                    points.push((nx, ny));
                }
            }
        }
    }
    points
}

