// --- 共通設定 ---
pub const LAYER_SIZE: usize = 4;
pub const DEBOUNCE: u32 = 5;

pub const USB_POLLING_INTERVAL_MS: u8 = 1;
pub const USB_VENDOR_ID: u16 = 0xFEED;
pub const USB_PRODUCT_ID: u16 = 0x0001;
pub const USB_DEVICE_VERSION: u16 = 0x0001; // 0x0001 = v0.0.1
pub const USB_MANUFACTURER: &str = "DHW";
pub const USB_PRODUCT: &str = "Mira Keyboard";

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DiodeDirection {
    Col2Row,
    Row2Col,
}
pub const DIODE_DIRECTION: DiodeDirection = DiodeDirection::Col2Row;

// --- 分割キーボード用の設定 (split フィーチャー有効時) ---
#[cfg(feature = "split")]
pub const IS_SPLIT: bool = true;
#[cfg(feature = "split")]
pub const MATRIX_ROWS: usize = 4;
#[cfg(feature = "split")]
pub const MATRIX_COLS: usize = 5;
#[cfg(feature = "split")]
pub const MATRIX_COLS_TOTAL: usize = 10; // 左右合わせた論理列数

#[cfg(all(feature = "split", feature = "left"))]
mod hand_config {
    pub const IS_LEFT: bool = true;
    pub const ROW_PINS: [usize; super::MATRIX_ROWS] = [4, 5, 6, 7];
    pub const COL_PINS: [usize; super::MATRIX_COLS] = [29, 28, 27, 26, 15];
}

#[cfg(all(feature = "split", feature = "right"))]
mod hand_config {
    pub const IS_LEFT: bool = false;
    pub const ROW_PINS: [usize; super::MATRIX_ROWS] = [0, 1, 2, 3];
    pub const COL_PINS: [usize; super::MATRIX_COLS] = [29, 28, 27, 26, 15];
}

// --- 一体型キーボード用の設定 (split フィーチャー無効時) ---
#[cfg(not(feature = "split"))]
pub const IS_SPLIT: bool = false;
#[cfg(not(feature = "split"))]
pub const MATRIX_ROWS: usize = 2;
#[cfg(not(feature = "split"))]
pub const MATRIX_COLS: usize = 10;
#[cfg(not(feature = "split"))]
pub const MATRIX_COLS_TOTAL: usize = 10;

#[cfg(not(feature = "split"))]
mod hand_config {
    pub const IS_LEFT: bool = true;
    pub const ROW_PINS: [usize; super::MATRIX_ROWS] = [10, 9];
    pub const COL_PINS: [usize; super::MATRIX_COLS] = [0, 1, 2, 3, 4, 15, 13, 27, 28, 29];
}

pub use hand_config::*;

// 左右間シリアル通信 (GP11) 用のピン（現在はPIOシリアル通信で直接GP11を指定しているため未使用）
pub const UART_TX_PIN: usize = 11;
pub const UART_RX_PIN: usize = 11;
