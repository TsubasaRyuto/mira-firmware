use crate::board::Board;
use crate::config::{MATRIX_COLS, MATRIX_ROWS};
use crate::debounce::Debouncer;
use crate::keycodes::Action;
use crate::keymaps::default::DEFAULT;
use crate::layers::*;
use crate::usb::REPORT_CHANNEL;

use embassy_time::{Duration, Instant, Timer};
use usbd_hid::descriptor::KeyboardReport;

pub struct Matrix {
    keys_debouncer: [[Debouncer; MATRIX_COLS]; MATRIX_ROWS],
    /// 各キーが押された時に決定されたアクションを保持する
    keys_action: [[Action; MATRIX_COLS]; MATRIX_ROWS],
    board: Board,
    layer: Layer,
    last_report: KeyboardReport,
}

impl Matrix {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            keys_debouncer: [[Debouncer::new(); MATRIX_COLS]; MATRIX_ROWS],
            keys_action: [[Action::NoOp; MATRIX_COLS]; MATRIX_ROWS],
            layer: Layer::new(),
            last_report: KeyboardReport {
                modifier: 0,
                reserved: 0,
                leds: 0,
                keycodes: [0; 6],
            },
        }
    }

    async fn scan(&mut self) {
        let current_time = Instant::now();

        for row_pin in self.board.rows.iter_mut() {
            row_pin.set_high();
        }

        for (r_idx, row_pin) in self.board.rows.iter_mut().enumerate() {
            row_pin.set_low();
            Timer::after_micros(1).await;

            for (c_idx, col_pin) in self.board.cols.iter_mut().enumerate() {
                let pressed = col_pin.is_low();
                self.keys_debouncer[r_idx][c_idx].update(pressed, current_time);
            }

            row_pin.set_high();
        }
    }

    pub fn build_report(&mut self) -> KeyboardReport {
        let mut keycodes = [0u8; 6];
        let mut modifier = 0x00;
        let mut idx = 0;

        for r in 0..MATRIX_ROWS {
            for c in 0..MATRIX_COLS {
                let debouncer = &self.keys_debouncer[r][c];

                // 押された瞬間にアクションを確定させ、保持する
                if debouncer.just_pressed() {
                    let action = self.get_action(r, c);
                    self.keys_action[r][c] = action;

                    match action {
                        Action::LayerTap { layer_idx: layer, .. }
                        | Action::LayerMomentary(layer) => {
                            self.layer.press(layer);
                        }
                        Action::LayerToggle(layer) => {
                            if (self.layer.state >> layer) & 1 == 1 {
                                self.layer.release(layer);
                            } else {
                                self.layer.press(layer);
                            }
                        }
                        _ => {}
                    }
                }

                // 保持されているアクションを取得（押下中・離した時もこれを使用する）
                let action = self.keys_action[r][c];

                if debouncer.just_released() {
                    match action {
                        Action::LayerTap { layer_idx: layer, .. }
                        | Action::LayerMomentary(layer) => {
                            self.layer.release(layer);
                        }
                        _ => {}
                    }
                    // 離した後はアクションをクリア
                    self.keys_action[r][c] = Action::NoOp;
                }

                if debouncer.is_pressed() {
                    match action {
                        Action::Key(key_code) => {
                            if idx < keycodes.len() {
                                keycodes[idx] = key_code;
                                idx += 1;
                            }
                        }
                        Action::Modifier(modifier_code)
                        | Action::ModTap { modifier: modifier_code, .. } => {
                            modifier |= modifier_code;
                        }
                        Action::LayerTap { key_code, .. } => {
                            // LayerTapのTap動作（簡易実装：同時押し時はキーコード優先）
                            if idx < keycodes.len() {
                                keycodes[idx] = key_code;
                                idx += 1;
                            }
                        }
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
        // 有効なレイヤーのみを、インデックスの高い順に走査
        for layer in (0..=self.layer.active_layer).rev() {
            if (self.layer.state >> layer) & 1 == 1 {
                let action = DEFAULT[layer as usize][row][col];
                if !matches!(action, Action::Trans) {
                    return action;
                }
            }
        }
        Action::NoOp
    }
}

/// マトリックススキャンタスクのエントリポイント (impl の外に配置)
#[embassy_executor::task]
pub async fn matrix_task(mut matrix: Matrix) -> ! {
    loop {
        matrix.scan().await;
        let report = matrix.build_report();

        if report.modifier != matrix.last_report.modifier
            || report.keycodes != matrix.last_report.keycodes
        {
            if REPORT_CHANNEL.try_send(report).is_ok() {
                matrix.last_report = report;
            }
        }

        Timer::after(Duration::from_millis(1)).await;
    }
}
