#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameScreen {
    Menu,
    Playing,
    GameOver,
}

#[derive(Clone, Copy, Debug)]
pub enum DeathReason {
    HitWall,
    HitSelf,
}

// Grid constants
pub const GRID_SIZE: i32 = 8;  // Size of each grid cell in pixels
pub const GRID_WIDTH: i32 = 128 / GRID_SIZE;   // 16 cells wide
pub const GRID_HEIGHT: i32 = 64 / GRID_SIZE;   // 8 cells tall
