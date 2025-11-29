#[derive(Copy, Clone)]
pub enum Action {
    Key(KeyCode),
    Modifier(Modifier),
    LayerTap(usize, KeyCode),
    TapHoldLayer(usize),
    Trans,
    NoOp,
}

/// USB HID Usage ID.
pub type KeyCode = u8;
pub type Modifier = u8;

pub mod kc {
    use super::KeyCode;

    pub const ROLLOVER: KeyCode = 0x01;

    // 文字キー
    pub const A: KeyCode = 0x04;
    pub const B: KeyCode = 0x05;
    pub const C: KeyCode = 0x06;
    pub const D: KeyCode = 0x07;
    pub const E: KeyCode = 0x08;
    pub const F: KeyCode = 0x09;
    pub const G: KeyCode = 0x0A;
    pub const H: KeyCode = 0x0B;
    pub const I: KeyCode = 0x0C;
    pub const J: KeyCode = 0x0D;
    pub const K: KeyCode = 0x0E;
    pub const L: KeyCode = 0x0F;
    pub const M: KeyCode = 0x10;
    pub const N: KeyCode = 0x11;
    pub const O: KeyCode = 0x12;
    pub const P: KeyCode = 0x13;
    pub const Q: KeyCode = 0x14;
    pub const R: KeyCode = 0x15;
    pub const S: KeyCode = 0x16;
    pub const T: KeyCode = 0x17;
    pub const U: KeyCode = 0x18;
    pub const V: KeyCode = 0x19;
    pub const W: KeyCode = 0x1A;
    pub const X: KeyCode = 0x1B;
    pub const Y: KeyCode = 0x1C;
    pub const Z: KeyCode = 0x1D;

    // 数字
    pub const _1: KeyCode = 0x1E;
    pub const _2: KeyCode = 0x1F;
    pub const _3: KeyCode = 0x20;
    pub const _4: KeyCode = 0x21;
    pub const _5: KeyCode = 0x22;
    pub const _6: KeyCode = 0x23;
    pub const _7: KeyCode = 0x24;
    pub const _8: KeyCode = 0x25;
    pub const _9: KeyCode = 0x26;
    pub const _0: KeyCode = 0x27;

    // 制御系
    pub const ENTER: KeyCode = 0x28;
    pub const ESC: KeyCode = 0x29;
    pub const BACKSP: KeyCode = 0x2A;
    pub const TAB: KeyCode = 0x2B;
    pub const SPACE: KeyCode = 0x2C;

    // 記号（US）
    pub const MINUS: KeyCode = 0x2D; // -
    pub const EQUAL: KeyCode = 0x2E; // =
    pub const LBRACKET: KeyCode = 0x2F; // [
    pub const RBRACKET: KeyCode = 0x30; // ]
    pub const BSLASH: KeyCode = 0x31; // \
    pub const SCOLON: KeyCode = 0x33; // ;
    pub const QUOTE: KeyCode = 0x34; // '
    pub const GRAVE: KeyCode = 0x35; // `
    pub const COMMA: KeyCode = 0x36; // ,
    pub const DOT: KeyCode = 0x37; // .
    pub const SLASH: KeyCode = 0x38; // /

    // 矢印
    pub const RIGHT: KeyCode = 0x4F;
    pub const LEFT: KeyCode = 0x50;
    pub const DOWN: KeyCode = 0x51;
    pub const UP: KeyCode = 0x52;

    // ファンクションキー
    pub const F1: KeyCode = 0x3A;
    pub const F2: KeyCode = 0x3B;
    pub const F3: KeyCode = 0x3C;
    pub const F4: KeyCode = 0x3D;
    pub const F5: KeyCode = 0x3E;
    pub const F6: KeyCode = 0x3F;
    pub const F7: KeyCode = 0x40;
    pub const F8: KeyCode = 0x41;
    pub const F9: KeyCode = 0x42;
    pub const F10: KeyCode = 0x43;
    pub const F11: KeyCode = 0x44;
    pub const F12: KeyCode = 0x45;
}

// 修飾キー
pub mod modifiers {
    use super::Modifier;

    pub const LCTRL: Modifier = 0b0000_0001;
    pub const LSHIFT: Modifier = 0b0000_0010;
    pub const LALT: Modifier = 0b0000_0100;
    pub const LGUI: Modifier = 0b0000_1000;
    pub const RCTRL: Modifier = 0b0001_0000;
    pub const RSHIFT: Modifier = 0b0010_0000;
    pub const RALT: Modifier = 0b0100_0000;
    pub const RGUI: Modifier = 0b1000_0000;
}
