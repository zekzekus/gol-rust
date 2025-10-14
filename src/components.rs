#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    pub alive: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NextCell {
    pub alive: bool,
}
