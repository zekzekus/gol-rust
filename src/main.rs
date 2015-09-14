use std::collections::BTreeMap;
use std::thread::sleep_ms;

// a sorted tree map to represent whole game board
// coordinated by tuples of integers. Dead or Alive status 
// 1 and 0.
type Grid = BTreeMap<(i32, i32), i32>;

#[derive(Debug)]
struct World {
    width: i32,
    height: i32,
    grid: Grid,
}

impl World {
    fn new(width: i32, height: i32) -> World {
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
        World{width: width, height: height, grid: grid}
    }

    fn print(&self) {
        let mut tx = 0;
        for (key, value) in self.grid.iter() {
            if key.0 != tx {
                tx = key.0;
                print!("\n");
            }
            if *value == 1 {
                print!("#");
            } else {
                print!("-");
            }
        }
        print!("\n");
    }

    fn next(self) -> World {
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

fn main() {
    let mut world: World = World::new(47, 94);
    println!("initial world");
    world.print();
    for i in 1..50 {
        println!("{}. iteration", i);
        world = world.next();
        world.print();
        sleep_ms(50);
    }
}
