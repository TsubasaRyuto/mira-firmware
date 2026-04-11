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
    board: Board,
    layer: Layer,
    last_report: KeyboardReport,
}

impl Matrix {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            keys_debouncer: [[Debouncer::new(); MATRIX_COLS]; MATRIX_ROWS],
            layer: Layer::new(),
            last_report: KeyboardReport {
                modifier: 0,
                reserved: 0,
                leds: 0,
                keycodes: [0; 6],
            },
        }
    }

    /// マトリックススキャンタスクのエントリポイント
    #[embassy_executor::task]
    pub async fn run(mut matrix: Matrix) -> ! {
        loop {
            // 物理スキャン
            matrix.scan().await;

            // レポート作成
            let report = matrix.build_report();

            // 状態が変化したときのみ送信
            if report.modifier != matrix.last_report.modifier
                || report.keycodes != matrix.last_report.keycodes
            {
                if REPORT_CHANNEL.try_send(report).is_ok() {
                    matrix.last_report = report;
                }
            }

            // 1ms待機（1000Hzスキャン）
            Timer::after(Duration::from_millis(1)).await;
        }
    }

    async fn scan(&mut self) {
        let current_time = Instant::now();

        for row_pin in self.board.rows.iter_mut() {
            row_pin.set_high();
        }

        for (r_idx, row_pin) in self.board.rows.iter_mut().enumerate() {
            row_pin.set_low();
            
            // 寄生容量などによる信号の立ち上がり/立ち下がりを待つ (極短時間)
            // RP2040のような高速MCUでは数クロック必要だが、yieldするほどではないため
            // ここでは簡易的なウェイトを入れるか、直後の読み取りで対応
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
                let action = self.get_action(r, c);

                // イベント処理
                if debouncer.just_pressed() {
                    match action {
                        Action::LayerTap { layer_idx: layer, .. }
                        | Action::LayerMomentary(layer) => {
                            self.layer.update(layer);
                        }
                        _ => {}
                    }
                }

                if debouncer.just_released() {
                    match action {
                        Action::LayerTap { .. } | Action::LayerMomentary(_) => {
                            self.layer.update(LAYER0);
                        }
                        _ => {}
                    }
                }

                // 状態処理
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
