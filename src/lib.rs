pub mod seeder;

pub use seeder::Seeder;

use std::collections::BTreeMap;

// a sorted tree map to represent whole game board
// coordinated by tuples of integers. Dead or Alive status 
// 1 and 0.
pub type Grid = BTreeMap<(i32, i32), i32>;

pub struct World {
    width: i32,
    height: i32,
    grid: Grid,
}

impl World {
    pub fn new(width: i32, height: i32, seeder: seeder::Seeder) -> Self {
        let grid = seeder.seed(width, height);
        World{width: width, height: height, grid: grid}
    }

    pub fn print(&self) {
        let mut tx = 0;
        for (key, value) in self.grid.iter() {
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
        for (key, state) in self.grid.iter() {
            let mut total_state: i32 = 0;
            let mut new_state: i32 = *state;
            for n in neighbours(*key, self.width, self.height) {
                let state = self.grid.get(&n).unwrap();
                total_state += *state;
            }
            if *state == 1 && (total_state < 2 || total_state > 3) {
                new_state = 0;
            } else if *state == 0 && total_state == 3 {
                new_state = 1;
            }
            next_grid.insert(*key, new_state);
        }
        World{width: self.width, height: self.height, grid: next_grid}
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

