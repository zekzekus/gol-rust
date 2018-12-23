use rand;

use super::Grid;

use self::rand::{thread_rng, Rng};

pub enum Seeder {
    Random,
    Glider,
    CenterOne,
    CenterFive,
}

impl Seeder {
    pub fn seed(&self, width: i32, height: i32) -> Grid {
        match *self {
            Seeder::Glider => grid_glider(width, height),
            Seeder::Random => grid_random(width, height),
            Seeder::CenterOne => grid_center_one(width, height),
            Seeder::CenterFive => grid_center_five(width, height),
        }
    }

    pub fn new(index: u32) -> Self {
        match index {
            0 => Seeder::Random,
            1 => Seeder::Glider,
            2 => Seeder::CenterOne,
            3 => Seeder::CenterFive,
            _ => panic!("invalid seeder: {}", index),
        }
    }
}

fn grid_glider(width: i32, height: i32) -> Grid {
    let mut grid = Grid::new();
    for x in 0..width {
        for y in 0..height {
            if (x == 1 && y == 2) || (x == 2 && y == 3) || (x == 3 && (y == 1 || y == 2 || y == 3)) {
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

fn grid_center_one(width: i32, height: i32) -> Grid {
    let mut grid = Grid::new();
    for row in 0..width {
        for col in 0..height {
            if row == width / 2 && col == height / 2 {
                grid.insert((row, col), 1);
            } else {
                grid.insert((row, col), 0);
            }
        }
    }
    grid
}

fn grid_center_five(width: i32, height: i32) -> Grid {
    let mut grid = Grid::new();
    for row in 0..width {
        for col in 0..height {
            if row == width / 2 && col >= height / 2 && col <= height / 2 + 5 {
                grid.insert((row, col), 1);
            } else {
                grid.insert((row, col), 0);
            }
        }
    }
    grid
}
