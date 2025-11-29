use crate::config::{LAYER_SIZE, MATRIX_COLS, MATRIX_ROWS};
use crate::keycodes::Action::*;
use crate::keycodes::{Action, kc, modifiers};
use crate::layers;

pub const DEFAULT: [[[Action; MATRIX_COLS]; MATRIX_ROWS]; LAYER_SIZE] =
    [LAYER0, LAYER1, LAYER2, LAYER3];

const LAYER0: [[Action; MATRIX_COLS]; MATRIX_ROWS] = [
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
        Key(kc::SCOLON),
    ],
    [
        NoOp,
        NoOp,
        NoOp,
        TapHoldLayer(layers::LAYER1),
        TapHoldLayer(layers::LAYER2),
        TapHoldLayer(layers::LAYER3),
        NoOp, // Was TapHoldLayer(layers::LAYER4), which causes a panic.
        NoOp,
        NoOp,
        NoOp,
    ],
];

const LAYER1: [[Action; MATRIX_COLS]; MATRIX_ROWS] = [
    [
        Key(kc::Q),
        Key(kc::W),
        Key(kc::E),
        Key(kc::R),
        Key(kc::T),
        Key(kc::Y),
        Key(kc::U),
        Key(kc::I),
        Key(kc::O),
        Key(kc::P),
    ],
    [
        NoOp, NoOp, NoOp, Trans, Trans, // Was Key(kc::SPACE), bug.
        Trans, // Was Key(kc::BACKSP), bug.
        Trans, // Was Key(kc::ENTER), bug.
        NoOp, NoOp, NoOp,
    ],
];

const LAYER2: [[Action; MATRIX_COLS]; MATRIX_ROWS] = [
    [
        Key(kc::Z),
        Key(kc::X),
        Key(kc::C),
        Key(kc::V),
        Key(kc::B),
        Key(kc::N),
        Key(kc::M),
        Trans,
        Trans,
        Trans,
    ],
    [
        NoOp, NoOp, NoOp, Trans, // Was Key(kc::ESC), bug.
        Trans, Trans, Trans, // Was Key(kc::SPACE), bug.
        NoOp, NoOp, NoOp,
    ],
];

const LAYER3: [[Action; MATRIX_COLS]; MATRIX_ROWS] = [
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
        NoOp, NoOp, NoOp, Trans, // Was Key(kc::ESC), bug.
        Trans, // Was Key(kc::SPACE), bug.
        Trans, Trans, NoOp, NoOp, NoOp,
    ],
];
