use crate::game::types::Direction;

// Should modify this based on the hardware
const LOW_THRESHOLD: u16 = 1000;
const HIGH_THRESHOLD: u16 = 3000;

pub fn process_joystick_input(vrx: u16, vry: u16) -> Option<Direction> {
    // Simple thresholds based on joystick readings ==> Need to improve this.
    if vrx < LOW_THRESHOLD {
        Some(Direction::Left)
    } else if vrx > HIGH_THRESHOLD {
        Some(Direction::Right)
    } else if vry < LOW_THRESHOLD {
        Some(Direction::Up)
    } else if vry > HIGH_THRESHOLD {
        Some(Direction::Down)
    } else {
        None
    }
}

pub fn check_button_press(current_state: bool, prev_state: &mut bool) -> bool {
    let pressed = current_state && !*prev_state;
    *prev_state = current_state;
    pressed
}
