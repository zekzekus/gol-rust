pub mod rules;
pub mod seeder;

pub use crate::rules::Rule;
pub use crate::seeder::Seeder;

use std::collections::BTreeMap;

use bracket_lib::prelude::*;

pub type Grid = BTreeMap<(i32, i32), i32>;

pub struct World {
    width: i32,
    height: i32,
    pub grid: Grid,
    rule: Rule,
}

impl World {
    pub fn new(width: i32, height: i32, seeder: &seeder::Seeder, rule: Rule) -> Self {
        let grid = seeder.seed(width, height);
        World {
            width,
            height,
            grid,
            rule,
        }
    }

    pub fn print(&self) {
        let mut tx = 0;
        for (key, value) in &self.grid {
            if key.0 != tx {
                tx = key.0;
                println!();
            }
            if *value == 1 {
                print!("O");
            } else {
                print!("-");
            }
        }
        println!();
    }

    pub fn next(&self) -> Self {
        let mut next_grid: Grid = Grid::new();

        for (key, state) in &self.grid {
            let mut total_state: i32 = 0;
            for n in neighbours(*key, self.width, self.height) {
                let state = self.grid[&n];
                total_state += state;
            }
            let new_state = self.rule.check(*state, total_state);
            next_grid.insert(*key, new_state);
        }
        World {
            width: self.width,
            height: self.height,
            grid: next_grid,
            rule: Rule {
                borns: self.rule.borns.clone(),
                stays: self.rule.stays.clone(),
            },
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.cls();
        for (key, value) in &self.grid {
            let disp = if *value == 1 { 'O' } else { ' ' };
            ctx.set(key.0, key.1, RGB::named(WHITE), RGB::named(BLACK), to_cp437(disp));
        }
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
