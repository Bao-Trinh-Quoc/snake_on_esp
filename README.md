# RustViper - Snake Game for ESP32 

A classic Snake game implementation written in Rust for the ESP32 microcontroller, featuring a 128x64 OLED display and joystick controls.

This is a pet project I built whenever I had some free time or got stuck at my day job.

## Demo video

https://github.com/user-attachments/assets/5fb0b047-d07d-487c-b3b9-d9bd26f1c59c


## Hardware 

### Components
- ESP32 development board
- SSD1306 128x64 OLED display (I2C)
- Analog joystick module

### Wiring
Could be changed based on your setup.

| Component | ESP32 Pin | Description |
|-----------|-----------|-------------|
| SSD1306 SDA | GPIO23 | I2C Data |
| SSD1306 SCL | GPIO18 | I2C Clock |
| SSD1306 VCC | 3.3V | Power |
| SSD1306 GND | GND | Ground |
| Joystick VRX | GPIO15 | X-axis analog input |
| Joystick VRY | GPIO2 | Y-axis analog input |
| Joystick SW | GPIO4 | Button input (with pull-up) |
| Joystick VCC | 3.3V | Power |
| Joystick GND | GND | Ground |

## Software
Refer to these to setup the ESP toolchain:
- [Rust on ESP](https://docs.espressif.com/projects/rust/book/introduction.html)
- [impl Rust](https://esp32.implrust.com/) 

## Game Controls

- **Joystick**: Move the snake (Up/Down/Left/Right)
- **Button**: 
  - Start game from menu
  - Restart after game over

## Project Structure

```
src/
├── bin/
│   └── main.rs          # Main application entry point
├── game/
│   ├── mod.rs           # Game module exports
│   ├── snake.rs         # Snake entity logic
│   ├── food.rs          # Food entity logic
│   └── types.rs         # Game types and constants
├── gamestate.rs         # Game state management
├── input.rs             # Input handling (joystick & button)
├── display.rs           # Display rendering
└── lib.rs               # Library root
```

## How to Run

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run with output
cargo run --release
```

## Troubleshooting

Note to myself: If somehow in the future you clone this and it doesn't work, check the following list:
-   **Start small**, try to blink an LED on ESP32 first <--- This is crucial because it helps verify that your toolchain and hardware setup are working correctly.
-   **Try to test each module following this order**:
    1. [OLED](https://esp32.implrust.com/oled/index.html)
    2. [Joystick](https://esp32.implrust.com/joystick/index.html)
    3. Integrate both modules.

## Room for Improvement

- The current source code could use embassy for task management and concurrent (checking for input at a faster rate than the game update rate) tasks but it requires messing with the lifetimes - a topic worth exploring further.
- Add more game features like levels, obstacles, or power-ups.
- Optimize stuff if needded

## References
- [The Rust book](https://doc.rust-lang.org/book/)
- [The Embedded Rust book](https://docs.rust-embedded.org/book/)
- [Impl Rust ESP32](https://esp32.implrust.com/)