#![no_std]
#![no_main]

mod config;
mod keycodes;
mod matrix;

use panic_halt as _;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    loop {}
}
