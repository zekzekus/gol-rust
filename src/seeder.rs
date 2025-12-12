use rand::{thread_rng, Rng};
use std::collections::BTreeMap;

pub type Grid = BTreeMap<(i32, i32), i32>;

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
            grid.insert((row, col), rng.gen_range(0..2));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seeder_new_random() {
        match Seeder::new(0) {
            Seeder::Random => assert!(true),
            _ => panic!("Expected Seeder::Random"),
        }
    }

    #[test]
    fn test_seeder_new_glider() {
        match Seeder::new(1) {
            Seeder::Glider => assert!(true),
            _ => panic!("Expected Seeder::Glider"),
        }
    }

    #[test]
    fn test_seeder_new_center_one() {
        match Seeder::new(2) {
            Seeder::CenterOne => assert!(true),
            _ => panic!("Expected Seeder::CenterOne"),
        }
    }

    #[test]
    fn test_seeder_new_center_five() {
        match Seeder::new(3) {
            Seeder::CenterFive => assert!(true),
            _ => panic!("Expected Seeder::CenterFive"),
        }
    }

    #[test]
    #[should_panic(expected = "invalid seeder: 99")]
    fn test_seeder_new_invalid() {
        Seeder::new(99);
    }

    #[test]
    fn test_grid_glider_dimensions() {
        let grid = grid_glider(10, 10);
        assert_eq!(grid.len(), 100);
    }

    #[test]
    fn test_grid_glider_pattern() {
        let grid = grid_glider(10, 10);
        assert_eq!(grid.get(&(1, 2)), Some(&1));
        assert_eq!(grid.get(&(2, 3)), Some(&1));
        assert_eq!(grid.get(&(3, 1)), Some(&1));
        assert_eq!(grid.get(&(3, 2)), Some(&1));
        assert_eq!(grid.get(&(3, 3)), Some(&1));
        assert_eq!(grid.get(&(0, 0)), Some(&0));
        assert_eq!(grid.get(&(5, 5)), Some(&0));
    }

    #[test]
    fn test_grid_random_dimensions() {
        let grid = grid_random(5, 7);
        assert_eq!(grid.len(), 35);
    }

    #[test]
    fn test_grid_random_values() {
        let grid = grid_random(10, 10);
        for (_, &value) in grid.iter() {
            assert!(value == 0 || value == 1);
        }
    }

    #[test]
    fn test_grid_center_one_dimensions() {
        let grid = grid_center_one(10, 10);
        assert_eq!(grid.len(), 100);
    }

    #[test]
    fn test_grid_center_one_pattern() {
        let grid = grid_center_one(10, 10);
        assert_eq!(grid.get(&(5, 5)), Some(&1));
        let alive_count = grid.values().filter(|&&v| v == 1).count();
        assert_eq!(alive_count, 1);
    }

    #[test]
    fn test_grid_center_five_dimensions() {
        let grid = grid_center_five(20, 20);
        assert_eq!(grid.len(), 400);
    }

    #[test]
    fn test_grid_center_five_pattern() {
        let width = 20;
        let height = 20;
        let grid = grid_center_five(width, height);

        for col in (height / 2)..=(height / 2 + 5) {
            assert_eq!(grid.get(&(width / 2, col)), Some(&1));
        }

        let alive_count = grid.values().filter(|&&v| v == 1).count();
        assert_eq!(alive_count, 6);
    }

    #[test]
    fn test_seeder_seed_glider() {
        let seeder = Seeder::Glider;
        let grid = seeder.seed(10, 10);
        assert_eq!(grid.get(&(1, 2)), Some(&1));
    }

    #[test]
    fn test_seeder_seed_center_one() {
        let seeder = Seeder::CenterOne;
        let grid = seeder.seed(10, 10);
        assert_eq!(grid.get(&(5, 5)), Some(&1));
    }
}
