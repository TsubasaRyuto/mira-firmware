use crate::matrix::DiodeDirection;

pub const MATRIX_ROWS: usize = 2;
pub const MATRIX_COLS: usize = 10;

pub const DIODE_DIRECTION: DiodeDirection = DiodeDirection::Col2Row;

pub const DEBOUNCE: usize = 10;

pub const USB_POLLING_INTERVAL_MS: usize = 1;
