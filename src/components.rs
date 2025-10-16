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
pub struct Age {
    pub value: u32,
}

impl Default for Age {
    fn default() -> Self {
        Age { value: 0 }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_age_default() {
        let age = Age::default();
        assert_eq!(age.value, 0);
    }

    #[test]
    fn test_cell_color_default() {
        let color = CellColor::default();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 255);
        assert_eq!(color.b, 255);
    }

    #[test]
    fn test_position_creation() {
        let pos = Position { x: 5, y: 10 };
        assert_eq!(pos.x, 5);
        assert_eq!(pos.y, 10);
    }

    #[test]
    fn test_cell_creation() {
        let cell = Cell { alive: true };
        assert_eq!(cell.alive, true);
        
        let dead_cell = Cell { alive: false };
        assert_eq!(dead_cell.alive, false);
    }

    #[test]
    fn test_age_creation() {
        let age = Age { value: 42 };
        assert_eq!(age.value, 42);
    }
}
