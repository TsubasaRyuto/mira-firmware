use crate::config::DEBOUNCE;

use rp2040_hal::fugit::{Duration, ExtU32, Instant};

const TIMER_HZ: u32 = 1_000_000;
type TimerInstant = Instant<u64, 1, TIMER_HZ>;

#[derive(Clone, Copy)]
pub struct Debouncer {
    pub stable_pressed: bool,
    cooldown_until: TimerInstant,
}

impl Debouncer {
    pub fn new() -> Self {
        Self {
            stable_pressed: false,
            cooldown_until: TimerInstant::from_ticks(0),
        }
    }

    pub fn update(&mut self, raw_input: bool, current_time: TimerInstant) -> bool {
        if self.cooldown_until > current_time {
            return false;
        }

        if raw_input != self.stable_pressed {
            let cooldown_duration: Duration<u64, 1, TIMER_HZ> = DEBOUNCE.millis().into();

            self.stable_pressed = raw_input;
            self.cooldown_until = current_time + cooldown_duration;

            return true;
        }

        false
    }
}
