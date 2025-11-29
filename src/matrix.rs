use crate::board::Board;
use crate::config::{MATRIX_COLS, MATRIX_ROWS};
use crate::debounce::Debouncer;
use crate::keycodes::Action;
use crate::keymaps::default::DEFAULT;
use crate::layers::*;

use embedded_hal::digital::{InputPin, OutputPin};
use usbd_hid::descriptor::KeyboardReport;

pub struct Matrix {
    keys_debouncer: [[Debouncer; MATRIX_COLS]; MATRIX_ROWS],
    board: Board,
    layer: Layer,
}

impl Matrix {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            keys_debouncer: [[Debouncer::new(); MATRIX_COLS]; MATRIX_ROWS],
            layer: Layer::new(),
        }
    }
    pub fn scan_matrix(&mut self) {
        let current_time = self.board.timer.get_counter();

        for row_pin in self.board.rows.iter_mut() {
            row_pin.set_high().ok();
        }

        for (r_idx, row_pin) in self.board.rows.iter_mut().enumerate() {
            row_pin.set_low().ok();

            for (c_idx, col_pin) in self.board.cols.iter_mut().enumerate() {
                let pressed = col_pin.is_low().unwrap_or(false);
                self.keys_debouncer[r_idx][c_idx].update(pressed, current_time);
            }

            row_pin.set_high().ok();
        }
    }

    pub fn build_report(&mut self) -> KeyboardReport {
        let mut keycodes = [0u8; 6];
        let mut modifier = 0x00;
        let mut idx = 0;

        for r in 0..MATRIX_ROWS {
            for c in 0..MATRIX_COLS {
                let debouncer = &self.keys_debouncer[r][c];
                let action = self.get_action(r, c);

                // --- イベント処理 (押した瞬間 / 離した瞬間) ---
                #[allow(clippy::collapsible_if)]
                if debouncer.just_pressed() {
                    if let Action::LayerTap(layer, _) | Action::TapHoldLayer(layer) = action {
                        self.layer.update(layer);
                    }
                }

                #[allow(clippy::collapsible_if)]
                if debouncer.just_released() {
                    if let Action::LayerTap(_, _) | Action::TapHoldLayer(_) = action {
                        self.layer.update(LAYER0);
                    }
                }

                // --- 状態処理 (押している間) ---
                if debouncer.is_pressed() {
                    match action {
                        Action::Key(key_code) => {
                            if idx < keycodes.len() {
                                keycodes[idx] = key_code;
                                idx += 1;
                            }
                        }
                        Action::Modifier(modifier_code) => {
                            modifier |= modifier_code;
                        }
                        // LayerTap や TapHoldLayer はイベント処理で完結しているので、
                        // ここでは何もしない。
                        _ => {}
                    }
                }
            }
        }

        KeyboardReport {
            modifier,
            reserved: 0,
            leds: 0,
            keycodes,
        }
    }

    fn get_action(&self, row: usize, col: usize) -> Action {
        for layer in (0..=self.layer.active_layer).rev() {
            let action = DEFAULT[layer][row][col];
            if !matches!(action, Action::Trans) {
                return action;
            }
        }
        Action::NoOp
    }
}
