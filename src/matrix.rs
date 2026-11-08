use crate::debounce::Debouncer;
use crate::keyboards::active_keyboard::board::Board;
use crate::keyboards::active_keyboard::config::{
    IS_LEFT, IS_SPLIT, MATRIX_COLS, MATRIX_COLS_TOTAL, MATRIX_ROWS,
};
use crate::keyboards::active_keyboard::keymaps::default::DEFAULT;
use crate::keycodes::Action;
use crate::layers::*;
use crate::usb::REPORT_CHANNEL;

use embassy_time::{Duration, Instant, Timer};
use usbd_hid::descriptor::KeyboardReport;

pub struct Matrix {
    keys_debouncer: [[Debouncer; MATRIX_COLS_TOTAL]; MATRIX_ROWS],
    /// 各キーが押された時に決定されたアクションを保持する
    keys_action: [[Action; MATRIX_COLS_TOTAL]; MATRIX_ROWS],
    /// 各キーが押された時刻
    keys_pressed_at: [[Option<Instant>; MATRIX_COLS_TOTAL]; MATRIX_ROWS],
    board: Board,
    layer: Layer,
    last_report: KeyboardReport,
}

impl Matrix {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            keys_debouncer: [[Debouncer::new(); MATRIX_COLS_TOTAL]; MATRIX_ROWS],
            keys_action: [[Action::NoOp; MATRIX_COLS_TOTAL]; MATRIX_ROWS],
            keys_pressed_at: [[None; MATRIX_COLS_TOTAL]; MATRIX_ROWS],
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

        // 物理スキャンの開始列 (一体型なら0、分割の左手なら0、右手なら5)
        let start_col = if !IS_SPLIT {
            0
        } else if IS_LEFT {
            0
        } else {
            MATRIX_COLS
        };

        for (r_idx, row_pin) in self.board.rows.iter_mut().enumerate() {
            row_pin.set_low();
            Timer::after_micros(1).await;

            for (c_idx, col_pin) in self.board.cols.iter_mut().enumerate() {
                let pressed = col_pin.is_low();
                self.keys_debouncer[r_idx][start_col + c_idx].update(pressed, current_time);
            }

            row_pin.set_high();
        }

        // 分割キーボードの場合のみ、キー状態の同期とマージを行う
        if IS_SPLIT {
            if IS_LEFT {
                // マスタ(左)はスレーブ(右)からの受信データを5..10列目に反映する
                let slave_mask = crate::serial::SLAVE_KEYS.load(core::sync::atomic::Ordering::Relaxed);
                for r in 0..MATRIX_ROWS {
                    for c in 0..MATRIX_COLS {
                        let pressed = (slave_mask & (1 << (r * MATRIX_COLS + c))) != 0;
                        self.keys_debouncer[r][5 + c].update(pressed, current_time);
                    }
                }
            } else {
                // スレーブ(右)は物理スキャン結果(5..10列目)をAtomic変数にパックして共有する
                let mut mask = 0u32;
                for r in 0..MATRIX_ROWS {
                    for c in 0..MATRIX_COLS {
                        if self.keys_debouncer[r][5 + c].is_pressed() {
                            mask |= 1 << (r * MATRIX_COLS + c);
                        }
                    }
                }
                crate::serial::MY_SLAVE_KEYS.store(mask, core::sync::atomic::Ordering::Relaxed);
            }
        }
    }

    pub fn build_report(&mut self) -> KeyboardReport {
        let mut keycodes = [0u8; 6];
        let mut modifier = 0x00;
        let mut idx = 0;
        let now = Instant::now();

        for r in 0..MATRIX_ROWS {
            for c in 0..MATRIX_COLS_TOTAL {
                let debouncer = &self.keys_debouncer[r][c];

                // 押された瞬間にアクションを確定させ、保持する
                if debouncer.just_pressed() {
                    let action = self.get_action(r, c);
                    self.keys_action[r][c] = action;
                    self.keys_pressed_at[r][c] = Some(now);

                    match action {
                        Action::LayerMomentary(layer) => {
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

                // 保持されているアクションを取得
                let action = self.keys_action[r][c];
                let pressed_at = self.keys_pressed_at[r][c];

                if debouncer.just_released() {
                    if let Some(t) = pressed_at {
                        if now.duration_since(t).as_millis() < 200 {
                            match action {
                                Action::LayerTap { key_code, .. }
                                | Action::ModTap { key_code, .. } => {
                                    if idx < keycodes.len() {
                                        keycodes[idx] = key_code;
                                        idx += 1;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }

                    match action {
                        Action::LayerTap {
                            layer_idx: layer, ..
                        }
                        | Action::LayerMomentary(layer) => {
                            self.layer.release(layer);
                        }
                        _ => {}
                    }

                    self.keys_action[r][c] = Action::NoOp;
                    self.keys_pressed_at[r][c] = None;
                }

                if debouncer.is_pressed() {
                    if let Some(t) = pressed_at {
                        match action {
                            Action::Key(key_code) => {
                                if idx < keycodes.len() {
                                    keycodes[idx] = key_code;
                                    idx += 1;
                                }
                            }
                            Action::Shifted(_) | Action::ModKey { .. } => {
                                let (mod_code, key_code) = match action {
                                    Action::ModKey { modifier, key_code } => (modifier, key_code),
                                    Action::Shifted(k) => (crate::keycodes::modifiers::LSHIFT, k),
                                    _ => unreachable!(),
                                };
                                // 押した瞬間のレポートは修飾キーのみ送り、次のレポートからキーを追加する。
                                // (同一レポートで送ると macOS が修飾キーを取りこぼすことがあるため)
                                modifier |= mod_code;
                                if t != now && idx < keycodes.len() {
                                    keycodes[idx] = key_code;
                                    idx += 1;
                                }
                            }
                            Action::Modifier(modifier_code) => {
                                modifier |= modifier_code;
                            }
                            Action::ModTap {
                                modifier: modifier_code,
                                ..
                            } => {
                                if now.duration_since(t).as_millis() >= 200 {
                                    modifier |= modifier_code;
                                }
                            }
                            Action::LayerTap {
                                layer_idx: layer, ..
                            } => {
                                if now.duration_since(t).as_millis() >= 200 {
                                    if (self.layer.state >> layer) & 1 == 0 {
                                        self.layer.press(layer);
                                    }
                                }
                            }
                            _ => {}
                        }
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

        if !IS_SPLIT || IS_LEFT {
            // 一体型、または左手（マスタ）のみキーイベントからレポートを作成して送信する
            let report = matrix.build_report();

            if report.modifier != matrix.last_report.modifier
                || report.keycodes != matrix.last_report.keycodes
            {
                if REPORT_CHANNEL.try_send(report).is_ok() {
                    matrix.last_report = report;
                }
            }
        }

        Timer::after(Duration::from_millis(1)).await;
    }
}
