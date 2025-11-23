use crate::board::Board;
use crate::config::{MATRIX_COLS, MATRIX_ROWS};
use crate::keymaps::default::DEFAULT;

use embedded_hal::digital::{InputPin, OutputPin};
use usbd_hid::descriptor::KeyboardReport;

pub struct Matrix {
    state: [[bool; MATRIX_COLS]; MATRIX_ROWS],
    board: Board,
}

impl Matrix {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            state: [[false; MATRIX_COLS]; MATRIX_ROWS],
        }
    }
    pub fn scan_matrix(&mut self) {
        Self::reset_state(self);

        // The pin is initialized low.
        // Therefore, all pins should be set to high.
        for row_pin in self.board.rows.iter_mut() {
            row_pin.set_high().ok();
        }

        for (r_idx, row_pin) in self.board.rows.iter_mut().enumerate() {
            row_pin.set_low().ok();

            for (c_idx, col_pin) in self.board.cols.iter_mut().enumerate() {
                let pressed = col_pin.is_low().unwrap_or(false);
                self.state[r_idx][c_idx] = pressed;
            }

            row_pin.set_high().ok();
        }
    }

    pub fn build_report(&mut self) -> KeyboardReport {
        let mut keycodes = [0u8; 6];
        let mut idx = 0;

        for r in 0..MATRIX_ROWS {
            for c in 0..MATRIX_COLS {
                // NOTE: The Keyboard report size is six bytes.
                // We break when the index reaches the report length.
                if idx == keycodes.len() {
                    break;
                }

                if self.state[r][c] {
                    keycodes[idx] = Self::get_keycode(r, c);
                    idx += 1;
                }
            }

            if idx == keycodes.len() {
                break;
            }
        }

        KeyboardReport {
            modifier: 0x00,
            reserved: 0,
            leds: 0,
            keycodes,
        }
    }

    fn reset_state(&mut self) {
        self.state = [[false; MATRIX_COLS]; MATRIX_ROWS];
    }

    fn get_keycode(row: usize, col: usize) -> u8 {
        DEFAULT[row][col]
    }
}
