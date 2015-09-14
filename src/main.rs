use std::collections::BTreeMap;

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

fn main() {
    let world: World = World::new(3, 3);
    println!("Hello, world!");
    println!("{:?}", world);
    world.print();
}
