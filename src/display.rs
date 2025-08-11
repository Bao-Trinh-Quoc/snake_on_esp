use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Rectangle, PrimitiveStyle},
    draw_target::DrawTarget,
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    text::{Baseline, Text},
};
use crate::game::{Snake, Food, types::{GRID_SIZE, GameScreen, DeathReason}};
use crate::gamestate::GameState;

pub async fn render_menu<D>(display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor> + OriginDimensions,
{
    display.clear(BinaryColor::Off)?;
    
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();
    
    Text::with_baseline("RustViper v1.0", Point::new(25, 10), text_style, Baseline::Top)
        .draw(display)?;
    
    Text::with_baseline("A game by Cloudy", Point::new(20, 25), text_style, Baseline::Top)
        .draw(display)?;
    
    Text::with_baseline("Click to play!", Point::new(30, 50), text_style, Baseline::Top)
        .draw(display)?;
    
    Ok(())
}

pub async fn render_game_over<D>(display: &mut D, score: u32, death_reason: DeathReason) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor> + OriginDimensions,
{
    display.clear(BinaryColor::Off)?;
    
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();
    
    let reason_text = match death_reason {
        DeathReason::HitWall => "You hit a wall",
        DeathReason::HitSelf => "You hit yourself",
    };
    
    Text::with_baseline("GAME OVER!", Point::new(40, 5), text_style, Baseline::Top)
        .draw(display)?;
    
    Text::with_baseline(reason_text, Point::new(30, 20), text_style, Baseline::Top)
        .draw(display)?;
    
    let mut buffer = itoa::Buffer::new();
    let score_text = buffer.format(score);

    Text::with_baseline(score_text, Point::new(64, 35), text_style, Baseline::Top)
        .draw(display)?;
    
    Text::with_baseline("Click to continue", Point::new(20, 50), text_style, Baseline::Top)
        .draw(display)?;
    
    Ok(())
}

pub async fn render_game<D>(display: &mut D, snake: &Snake, food: &Food) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor> + OriginDimensions,
{
    // Clear display
    display.clear(BinaryColor::Off)?;
    
    // Draw food (smaller rectangle)
    let food_pixel_x = food.position.x * GRID_SIZE;
    let food_pixel_y = food.position.y * GRID_SIZE;
    
    let food_square = Rectangle::new(
        Point::new(food_pixel_x + 2, food_pixel_y + 2),
        Size::new((GRID_SIZE - 4) as u32, (GRID_SIZE - 4) as u32),
    );
    food_square
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(display)?;
    
    // Draw snake
    for segment in snake.body.iter() {
        let pixel_x = segment.x * GRID_SIZE;
        let pixel_y = segment.y * GRID_SIZE;
        
        let square = Rectangle::new(
            Point::new(pixel_x, pixel_y),
            Size::new(GRID_SIZE as u32, GRID_SIZE as u32),
        );
        square
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(display)?;
    }
    
    Ok(())
}

pub async fn render_current_screen<D>(display: &mut D, game_state: &GameState) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor> + OriginDimensions,
{
    match game_state.current_screen {
        GameScreen::Menu => render_menu(display).await,
        GameScreen::Playing => render_game(display, &game_state.snake, &game_state.food).await,
        GameScreen::GameOver => {
            if let Some(reason) = game_state.death_reason {
                render_game_over(display, game_state.score, reason).await
            } else {
                render_menu(display).await // Fallback incase error (we change this)
            }
        }
    }
}