#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::analog::adc::{Adc, AdcConfig, Attenuation};
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Input, InputConfig, Pull};
use esp_hal::timer::timg::TimerGroup;
use esp_hal::time::Rate;
use esp_hal::rng::Rng;
use esp_println::{self as _, println};
use ssd1306::{prelude::*, size::DisplaySize128x64, I2CDisplayInterface, Ssd1306Async};
use esp_backtrace as _;

// Import our game modules
use snake::{GameState, input, display};

// This creates a default app-descriptor required by the esp-idf bootloader.
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    println!("RustViper v1.0");

    // Initialize I2C for display
    let i2c_bus = esp_hal::i2c::master::I2c::new(
        peripherals.I2C0,
        esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(400)),
    )
    .unwrap()
    .with_scl(peripherals.GPIO18)
    .with_sda(peripherals.GPIO23)
    .into_async();

    // Initialize display
    let interface = I2CDisplayInterface::new(i2c_bus);
    let mut ssd1306_display = Ssd1306Async::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    ssd1306_display.init().await.unwrap();

    // Initialize input hardware
    let button = Input::new(peripherals.GPIO4, InputConfig::default().with_pull(Pull::Up));
    let mut adc2_config = AdcConfig::new();
    let mut vrx_pin = adc2_config.enable_pin(peripherals.GPIO15, Attenuation::_11dB);
    let mut vry_pin = adc2_config.enable_pin(peripherals.GPIO2, Attenuation::_11dB);
    let mut adc2 = Adc::new(peripherals.ADC2, adc2_config);
    let mut prev_button_state = false;

    // Initialize random number generator
    let mut rng = Rng::new(peripherals.RNG);

    // Initialize game state
    let mut game_state = GameState::new(&mut rng);

    // Main game loop
    loop {
        // Read inputs
        let Ok(vrx) = nb::block!(adc2.read_oneshot(&mut vrx_pin)) else {
            continue;
        };
        let Ok(vry) = nb::block!(adc2.read_oneshot(&mut vry_pin)) else {
            continue;
        };
        

        let button_pressed = input::check_button_press(button.is_low(), &mut prev_button_state);
        let joystick_direction = input::process_joystick_input(vrx, vry);

        // Handle inputs
        if button_pressed {
            game_state.handle_button_press(&mut rng);
        }
        
        if game_state.is_playing() {
            game_state.handle_input(joystick_direction);
        }
        // Update game state
        game_state.update(&mut rng);
        
        // Render game
        if let Err(e) = display::render_current_screen(&mut ssd1306_display, &game_state).await {
            println!("Display error: {:?}", e);
        }
        
        // Flush display buffer
        ssd1306_display.flush().await.unwrap();
        
        // Debug output
        // println!("Score: {} | Snake head: ({}, {}) | Food: ({}, {}) | Length: {}", 
        //         game_state.score, game_state.snake.body[0].x, game_state.snake.body[0].y, 
        //         game_state.food.position.x, game_state.food.position.y, game_state.snake.len());

        Timer::after(Duration::from_millis(300)).await;
    }
}
