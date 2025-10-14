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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CellColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Default for CellColor {
    fn default() -> Self {
        CellColor { r: 255, g: 255, b: 255 }
    }
}
