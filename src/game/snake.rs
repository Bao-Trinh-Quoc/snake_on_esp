use heapless::Vec;
use crate::game::types::{Position, Direction, GRID_WIDTH, GRID_HEIGHT};

pub struct Snake {
    pub body: Vec<Position, 64>,
    pub direction: Direction,
    pub next_direction: Direction,
}

impl Snake {
    pub fn new() -> Self {
        let mut body: Vec<Position, 64> = Vec::new();
        // Init game with 3 block --- as a normal snake game would do
        body.push(Position { x: GRID_WIDTH / 2, y: GRID_HEIGHT / 2 }).unwrap();
        body.push(Position { x: GRID_WIDTH / 2 - 1, y: GRID_HEIGHT / 2 }).unwrap();
        body.push(Position { x: GRID_WIDTH / 2 - 2, y: GRID_HEIGHT / 2 }).unwrap();
        
        Self {
            body,
            direction: Direction::Right,
            next_direction: Direction::Right,
        }
    }
    
    pub fn reset(&mut self) {
        self.body.clear();
        self.body.push(Position { x: GRID_WIDTH / 2, y: GRID_HEIGHT / 2 }).unwrap();
        self.body.push(Position { x: GRID_WIDTH / 2 - 1, y: GRID_HEIGHT / 2 }).unwrap();
        self.body.push(Position { x: GRID_WIDTH / 2 - 2, y: GRID_HEIGHT / 2 }).unwrap();
        self.direction = Direction::Right;
        self.next_direction = Direction::Right;
    }
    
    pub fn set_direction(&mut self, new_direction: Direction) {
        // Prevent reversing into itself
        match (&self.direction, &new_direction) {
            (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) |
            (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => {}
            _ => self.next_direction = new_direction,
        }
    }
    
    pub fn get_next_head_position(&self) -> Position {
        let head = self.body[0];
        match self.next_direction {
            Direction::Up => Position { x: head.x, y: head.y - 1 },
            Direction::Down => Position { x: head.x, y: head.y + 1 },
            Direction::Left => Position { x: head.x - 1, y: head.y },
            Direction::Right => Position { x: head.x + 1, y: head.y },
        }
    }
    
    pub fn move_snake(&mut self, grow: bool) {
        self.direction = self.next_direction;
        let new_head = self.get_next_head_position();
        
        self.body.insert(0, new_head).unwrap();
        
        if !grow {
            self.body.pop();
        }
    }
    
    pub fn check_wall_collision(&self) -> bool {
        let head = self.body[0];
        head.x < 0 || head.x >= GRID_WIDTH || head.y < 0 || head.y >= GRID_HEIGHT
    }
    
    pub fn check_self_collision(&self) -> bool {
        let head = self.body[0];
        self.body.iter().skip(1).any(|segment| *segment == head)
    }
    
    pub fn contains_position(&self, pos: Position) -> bool {
        self.body.iter().any(|segment| *segment == pos)
    }
    
    pub fn len(&self) -> usize {
        self.body.len()
    }
}
