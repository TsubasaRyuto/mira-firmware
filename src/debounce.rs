use crate::keyboards::active_keyboard::config::DEBOUNCE;
use embassy_time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq, Eq, Debug, defmt::Format)]
pub enum KeyState {
    Released,
    JustPressed,
    Pressed,
    JustReleased,
}

#[derive(Clone, Copy)]
pub struct Debouncer {
    state: KeyState,
    cooldown_until: Instant,
}

impl Debouncer {
    pub const fn new() -> Self {
        Self {
            state: KeyState::Released,
            cooldown_until: Instant::from_ticks(0),
        }
    }

    pub fn update(&mut self, raw_input: bool, current_time: Instant) {
        if self.cooldown_until > current_time {
            // During cooldown, JustPressed/JustReleased transition to Pressed/Released
            if self.state == KeyState::JustPressed {
                self.state = KeyState::Pressed;
            } else if self.state == KeyState::JustReleased {
                self.state = KeyState::Released;
            }
            return;
        }

        let is_currently_pressed = matches!(self.state, KeyState::Pressed | KeyState::JustPressed);

        if raw_input != is_currently_pressed {
            // State has changed, set cooldown and update state
            self.cooldown_until = current_time + Duration::from_millis(DEBOUNCE as u64);

            self.state = if raw_input {
                KeyState::JustPressed
            } else {
                KeyState::JustReleased
            };
        } else {
            // State is stable, transition JustPressed/JustReleased to Pressed/Released
            if self.state == KeyState::JustPressed {
                self.state = KeyState::Pressed;
            } else if self.state == KeyState::JustReleased {
                self.state = KeyState::Released;
            }
        }
    }

    pub fn is_pressed(&self) -> bool {
        matches!(self.state, KeyState::Pressed | KeyState::JustPressed)
    }

    pub fn just_pressed(&self) -> bool {
        matches!(self.state, KeyState::JustPressed)
    }

    pub fn just_released(&self) -> bool {
        matches!(self.state, KeyState::JustReleased)
    }

    pub fn is_released(&self) -> bool {
        matches!(self.state, KeyState::Released | KeyState::JustReleased)
    }
}
