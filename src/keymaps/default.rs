use crate::config::{MATRIX_COLS, MATRIX_ROWS};
use crate::keycodes::{KeyCode, kc};

pub const DEFAULT: [[KeyCode; MATRIX_COLS]; MATRIX_ROWS] = [
    [
        kc::A,
        kc::S,
        kc::D,
        kc::F,
        kc::G,
        kc::H,
        kc::J,
        kc::K,
        kc::L,
        kc::SCOLON,
    ],
    [
        kc::NO,
        kc::NO,
        kc::NO,
        kc::ESC,
        kc::SPACE,
        kc::BACKSP,
        kc::ENTER,
        kc::NO,
        kc::NO,
        kc::NO,
    ],
];
