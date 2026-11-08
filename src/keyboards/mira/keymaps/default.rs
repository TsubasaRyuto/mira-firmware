use crate::keyboards::active_keyboard::config::{LAYER_SIZE, MATRIX_COLS_TOTAL, MATRIX_ROWS};
use crate::keycodes::Action::*;
use crate::keycodes::{Action, kc, modifiers};
use crate::layers;

// 33キー構成 (row0: 8, row1: 10, row2: 10, 親指: 左3 + 右2)
// row0 の col0/col9、row3 の col0,1,7,8,9 は物理キーなし (NoOp)
//
// L0 BASE : アルファベット / 親指 [SYM] [Cmd] [Space/NUM] | [Bksp] [Enter]
// L1 SYM  : 記号 / 親指 [SYM] [Cmd] [FN] | [Bksp] [Enter]
// L2 NUM  : 数字・矢印・Esc/Tab / 親指 [FN] [Cmd] [NUM] | [Bksp] [Enter]
// L3 FN   : SYM + Space/NUM 同時押しで F1〜F12

#[cfg(feature = "split")]
pub const DEFAULT: [[[Action; MATRIX_COLS_TOTAL]; MATRIX_ROWS]; LAYER_SIZE] =
    [LAYER0_SPLIT, LAYER1_SPLIT, LAYER2_SPLIT, LAYER3_SPLIT];

// ---------------------------------------------------------------------------
// L0 BASE
// ---------------------------------------------------------------------------
#[cfg(feature = "split")]
const LAYER0_SPLIT: [[Action; MATRIX_COLS_TOTAL]; MATRIX_ROWS] = [
    [
        NoOp,
        Key(kc::W),
        Key(kc::E),
        Key(kc::R),
        Key(kc::T),
        Key(kc::Y),
        Key(kc::U),
        Key(kc::I),
        Key(kc::O),
        NoOp,
    ],
    [
        Key(kc::A),
        Key(kc::S),
        Key(kc::D),
        Key(kc::F),
        Key(kc::G),
        Key(kc::H),
        Key(kc::J),
        Key(kc::K),
        Key(kc::L),
        Key(kc::P),
    ],
    [
        Modifier(modifiers::LSHIFT),
        ModTap { modifier: modifiers::LCTRL, key_code: kc::X }, // 長押し: Ctrl
        Key(kc::C),
        Key(kc::V),
        Key(kc::B),
        Key(kc::N),
        Key(kc::M),
        Key(kc::Z),
        ModTap { modifier: modifiers::RALT, key_code: kc::Q }, // 長押し: Alt(Option)
        Modifier(modifiers::RSHIFT), // 純粋な Shift のみ
    ],
    [
        NoOp,
        NoOp,
        LayerMomentary(layers::LAYER1), // SYM
        Modifier(modifiers::LGUI),      // Cmd
        LayerTap { layer_idx: layers::LAYER2, key_code: kc::SPACE }, // タップ: Space / 長押し: NUM
        Key(kc::BACKSP),                // Bksp
        Key(kc::ENTER),                 // Enter
        NoOp,
        NoOp,
        NoOp,
    ],
];

// ---------------------------------------------------------------------------
// L1 SYM
//       !    @    #    $   |   %    ^    &    *
// `     ~    (    )    -   |   _    [    ]   ;(:) '(")
// ⇧    \ |   {    }    =   |   +   ,(<) .(>) /(?)  ⇧
// ---------------------------------------------------------------------------
#[cfg(feature = "split")]
const LAYER1_SPLIT: [[Action; MATRIX_COLS_TOTAL]; MATRIX_ROWS] = [
    [
        NoOp,
        Shifted(kc::_1), // !
        Shifted(kc::_2), // @
        Shifted(kc::_3), // #
        Shifted(kc::_4), // $
        Shifted(kc::_5), // %
        Shifted(kc::_6), // ^
        Shifted(kc::_7), // &
        Shifted(kc::_8), // *
        NoOp,
    ],
    [
        Key(kc::GRAVE),        // `
        Shifted(kc::GRAVE),    // ~
        Shifted(kc::_9),       // (
        Shifted(kc::_0),       // )
        Key(kc::MINUS),        // -
        Shifted(kc::MINUS),    // _
        Key(kc::LBRACKET),     // [
        Key(kc::RBRACKET),     // ]
        Key(kc::SCOLON),       // ; (Shiftで :)
        Key(kc::QUOTE),        // ' (Shiftで ")
    ],
    [
        Trans,                 // 左手小指: LShift (BASE透過)
        Key(kc::BSLASH),       // \ (Shiftで |)
        Shifted(kc::LBRACKET), // {
        Shifted(kc::RBRACKET), // }
        Key(kc::EQUAL),        // =
        Shifted(kc::EQUAL),    // +
        Key(kc::COMMA),        // , (Shiftで <)
        Key(kc::DOT),          // . (Shiftで >)
        Key(kc::SLASH),        // / (Shiftで ?)
        Trans,                 // 右手小指: RShift / Enter (BASE透過)
    ],
    [
        NoOp,
        NoOp,
        Trans,                          // SYM (押下中)
        Trans,                          // Cmd
        LayerMomentary(layers::LAYER3), // Space/NUM位置 → FN
        Trans,                          // Bksp
        Trans,                          // Enter
        NoOp,
        NoOp,
        NoOp,
    ],
];

// ---------------------------------------------------------------------------
// L2 NUM / NAV
//    Esc Tab  ,   .  |  ←  ↓  ↑  →
// 1  2   3   4   5   |  6  7  8  9  0
// ⇧  <   >  Del  -   |  =  ,  .  /  ⇧Enter
// ---------------------------------------------------------------------------
#[cfg(feature = "split")]
const LAYER2_SPLIT: [[Action; MATRIX_COLS_TOTAL]; MATRIX_ROWS] = [
    [
        NoOp,
        Key(kc::ESC),
        Key(kc::TAB),
        Key(kc::COMMA),
        Key(kc::DOT),
        Key(kc::LEFT),
        Key(kc::DOWN),
        Key(kc::UP),
        Key(kc::RIGHT),
        NoOp,
    ],
    [
        Key(kc::_1),
        Key(kc::_2),
        Key(kc::_3),
        Key(kc::_4),
        Key(kc::_5),
        Key(kc::_6),
        Key(kc::_7),
        Key(kc::_8),
        Key(kc::_9),
        Key(kc::_0),
    ],
    [
        Trans,                // LShift
        Shifted(kc::COMMA),   // <
        Shifted(kc::DOT),     // >
        Key(kc::DELETE),      // 前方削除
        Key(kc::MINUS),       // -
        Key(kc::EQUAL),       // =
        Key(kc::COMMA),       // ,
        Key(kc::DOT),         // .
        Key(kc::SLASH),       // /
        Trans,                // Enter / RShift
    ],
    [
        NoOp,
        NoOp,
        LayerMomentary(layers::LAYER3), // SYM位置 → FN
        Trans,                          // Cmd (Cmd+矢印 = 行頭/行末)
        Trans,                          // Space/NUM (押下中)
        Trans,                          // Bksp
        Trans,                          // Enter
        NoOp,
        NoOp,
        NoOp,
    ],
];

// ---------------------------------------------------------------------------
// L3 FN (SYM + NUM)
//    F1  F2  F3  F4  |  F5  F6  F7  F8
// F9 F10 F11 F12  -  |  -   -   -   -   -
// ---------------------------------------------------------------------------
#[cfg(feature = "split")]
const LAYER3_SPLIT: [[Action; MATRIX_COLS_TOTAL]; MATRIX_ROWS] = [
    [
        NoOp,
        Key(kc::F1),
        Key(kc::F2),
        Key(kc::F3),
        Key(kc::F4),
        Key(kc::F5),
        Key(kc::F6),
        Key(kc::F7),
        Key(kc::F8),
        NoOp,
    ],
    [
        Key(kc::F9),
        Key(kc::F10),
        Key(kc::F11),
        Key(kc::F12),
        NoOp,
        NoOp,
        NoOp,
        NoOp,
        NoOp,
        NoOp,
    ],
    [NoOp, NoOp, NoOp, NoOp, NoOp, NoOp, NoOp, NoOp, NoOp, NoOp],
    [NoOp, NoOp, Trans, Trans, Trans, Trans, Trans, NoOp, NoOp, NoOp],
];
