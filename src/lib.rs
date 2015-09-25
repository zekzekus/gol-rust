#![feature(plugin)]
#![plugin(clippy)]
extern crate tcod;

pub mod seeder;
pub mod rules;

pub use seeder::Seeder;
pub use rules::Rule;

use std::collections::BTreeMap;

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
    rule: &'a Rule,
}

impl<'a> World<'a> {
    pub fn new(width: i32, height: i32, seeder: seeder::Seeder,
               rule: &'a Rule) -> Self {
        let grid = seeder.seed(width, height);
        World{width: width, height: height, grid: grid,
              rule: &rule}
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

        for (key, state) in &self.grid {
            let mut total_state: i32 = 0;
            for n in neighbours(*key, self.width, self.height) {
                let state = self.grid.get(&n).unwrap();
                total_state += *state;
            }
            let new_state = self.rule.check(*state, total_state);
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

