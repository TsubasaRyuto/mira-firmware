use crate::config::DEBOUNCE;
use rp2040_hal::fugit::{Duration, ExtU32, Instant};

const TIMER_HZ: u32 = 1_000_000;
type TimerInstant = Instant<u64, 1, TIMER_HZ>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KeyState {
    Released,
    JustPressed,
    Pressed,
    JustReleased,
}

#[derive(Clone, Copy)]
pub struct Debouncer {
    state: KeyState,
    cooldown_until: TimerInstant,
}

impl Debouncer {
    pub fn new() -> Self {
        Self {
            state: KeyState::Released,
            cooldown_until: TimerInstant::from_ticks(0),
        }
    }

    pub fn update(&mut self, raw_input: bool, current_time: TimerInstant) {
        let stable_pressed = match self.state {
            KeyState::Released | KeyState::JustReleased => false,
            KeyState::Pressed | KeyState::JustPressed => true,
        };

        if self.cooldown_until > current_time {
            return;
        }

        if raw_input == stable_pressed {
            // State is stable, transition JustPressed/JustReleased to Pressed/Released
            if self.state == KeyState::JustPressed {
                self.state = KeyState::Pressed;
            } else if self.state == KeyState::JustReleased {
                self.state = KeyState::Released;
            }
        } else {
            // State has changed, set cooldown and update state
            let cooldown_duration: Duration<u64, 1, TIMER_HZ> = DEBOUNCE.millis().into();
            self.cooldown_until = current_time + cooldown_duration;

            self.state = if raw_input {
                KeyState::JustPressed
            } else {
                KeyState::JustReleased
            };
        }
    }

    pub fn state(&self) -> KeyState {
        self.state
    }

    pub fn is_pressed(&self) -> bool {
        matches!(self.state, KeyState::Pressed)
    }

    pub fn just_pressed(&self) -> bool {
        matches!(self.state, KeyState::JustPressed)
    }

    pub fn just_released(&self) -> bool {
        matches!(self.state, KeyState::JustReleased)
    }

    pub fn is_released(&self) -> bool {
        matches!(self.state, KeyState::Released)
    }
}
