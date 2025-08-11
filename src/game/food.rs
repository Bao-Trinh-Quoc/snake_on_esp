use esp_hal::rng::Rng;
use crate::game::types::{Position, GRID_WIDTH, GRID_HEIGHT};
use crate::game::snake::Snake;

pub struct Food {
    pub position: Position,
}

impl Food {
    pub fn new(rng: &mut Rng) -> Self {
        Self {
            position: Position {
                x: (rng.random() as i32).abs() % GRID_WIDTH,
                y: (rng.random() as i32).abs() % GRID_HEIGHT,
            }
        }
    }
    
    pub fn spawn_new(&mut self, rng: &mut Rng, snake: &Snake) {
        // Keep trying until we find a position not occupied by the snake
        loop {
            self.position = Position {
                x: (rng.random() as i32).abs() % GRID_WIDTH,
                y: (rng.random() as i32).abs() % GRID_HEIGHT,
            };
            
            if !snake.contains_position(self.position) {
                break;
            }
        }
    }
}
