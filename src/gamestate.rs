use esp_hal::rng::Rng;
use esp_println::println;
use crate::game::{Snake, Food, types::{GRID_WIDTH, GRID_HEIGHT, GameScreen, DeathReason}};

pub struct GameState {
    pub snake: Snake,
    pub food: Food,
    pub score: u32,
    pub current_screen: GameScreen,
    pub death_reason: Option<DeathReason>,
}

impl GameState {
    pub fn new(rng: &mut Rng) -> Self {
        let snake = Snake::new();
        let food = Food::new(rng);
        
        Self {
            snake,
            food,
            score: 0,
            current_screen: GameScreen::Menu,
            death_reason: None,
        }
    }
    
    pub fn start_game(&mut self, rng: &mut Rng) {
        self.snake.reset();
        self.food.spawn_new(rng, &self.snake);
        self.score = 0;
        self.current_screen = GameScreen::Playing;
        self.death_reason = None;
        println!("Game started!");
    }
    
    pub fn update(&mut self, rng: &mut Rng) {
        if self.current_screen != GameScreen::Playing {
            return;
        }
        
        // Get next head position
        let next_head = self.snake.get_next_head_position();
        
        // Check for wall collision
        if next_head.x < 0 || next_head.x >= GRID_WIDTH || 
           next_head.y < 0 || next_head.y >= GRID_HEIGHT {
            self.death_reason = Some(DeathReason::HitWall);
            self.current_screen = GameScreen::GameOver;
            println!("Game Over! Hit wall. Score: {}", self.score);
            return;
        }
        
        // Check food collision
        let ate_food = next_head == self.food.position;
        
        // Move snake
        self.snake.move_snake(ate_food);
        
        // Check self collision after moving
        if self.snake.check_self_collision() {
            println!("Game Over! Hit self. Score: {} | Press button to restart", self.score);
            self.death_reason = Some(DeathReason::HitSelf);
            self.current_screen = GameScreen::GameOver;
            println!("Game Over! Hit self. Score: {}", self.score);
            return;
        }
        
        // Handle food consumption
        if ate_food {
            self.score += 1;
            println!("Food eaten! Score: {}", self.score);
            self.food.spawn_new(rng, &self.snake);
        }
    }
    
    pub fn handle_input(&mut self, direction: Option<crate::game::types::Direction>) {
        if self.current_screen == GameScreen::Playing {
            if let Some(dir) = direction {
                self.snake.set_direction(dir);
            }
        }
    }
    
    pub fn handle_button_press(&mut self, rng: &mut Rng) {
        match self.current_screen {
            GameScreen::Menu => {
                self.start_game(rng);
            },
            GameScreen::GameOver => {
                self.start_game(rng);
            },
            GameScreen::Playing => {
                // Button does nothing during gameplay
            }
        }
    }

    pub fn is_playing(&self) -> bool {
        self.current_screen == GameScreen::Playing
    }
}
