extern crate rand;

use super::Grid;

use self::rand::{thread_rng, Rng};

pub enum Seeder {
    Random,
    Glider,
}

impl Seeder {
    pub fn seed(&self, width: i32, height: i32) -> Grid {
        let grid = match *self {
            Seeder::Glider => grid_glider(width, height),
            Seeder::Random => grid_random(width, height),
        };
        grid
    }
}

fn grid_glider(width: i32, height: i32) -> Grid {
    let mut grid = Grid::new();
    for x in 0..width {
        for y in 0..height {
            if x == 1 && y == 2 {
                grid.insert((x, y), 1);
            } else if x == 2 && y == 3 {
                grid.insert((x, y), 1);
            } else if x == 3 && (y == 1 || y == 2 || y == 3) {
                grid.insert((x, y), 1);
            } else {
                grid.insert((x, y), 0);
            }
        }
    }
    grid
}

fn grid_random(width: i32, height: i32) -> Grid {
    let mut grid = Grid::new();
    let mut rng = thread_rng();
    for row in 0..width {
        for col in 0..height {
            grid.insert((row, col), rng.gen_range(0, 2));
        }
    }
    grid
}
