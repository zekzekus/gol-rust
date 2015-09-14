use std::collections::BTreeMap;

// A sorted tree map to hold game board.
// Key: tuple of integers to keep coordinates of a cell
// Value: Single char to represent state.
type Grid = BTreeMap<(i32, i32), char>;

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
                if x == 1 {
                    grid.insert((x, y), '#');
                } else {
                    grid.insert((x, y), '-');
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
            print!("{}", value);
        }
        print!("\n");
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
    let world: World = World::new(3, 3);
    println!("Hello, world!");
    println!("{:?}", world);
    world.print();
    println!("{:?}", neighbours((0, 0), 3, 3));
}
