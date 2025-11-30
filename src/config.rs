#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DiodeDirection {
    Col2Row,
    Row2Col,
}

pub const MATRIX_ROWS: usize = 2;
pub const MATRIX_COLS: usize = 10;
pub const LAYER_SIZE: usize = 4;

pub const DIODE_DIRECTION: DiodeDirection = DiodeDirection::Col2Row;

pub const DEBOUNCE: u32 = 5;

pub const USB_POLLING_INTERVAL_MS: u8 = 1;
pub const USB_VENDOR_ID: u16 = 0xFEED;
pub const USB_PRODUCT_ID: u16 = 0x0001;
pub const USB_DEVICE_VERSION: u16 = 0x0001; // 0x0001 = v0.0.1
pub const USB_MANUFACTURER: &str = "DHW";
pub const USB_PRODUCT: &str = "Mira Keyboard";

pub const ROW_PINS: [usize; MATRIX_ROWS] = [10, 9];
pub const COL_PINS: [usize; MATRIX_COLS] = [0, 1, 2, 3, 4, 15, 13, 27, 28, 29];
