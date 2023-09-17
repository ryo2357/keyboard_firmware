// 更新日:22023-08-06
// 親指キーの機能検証レイアウト
// 親指周辺のキーを持て余している

use crate::keycodes::*;
use crate::{KeyMap, KeyMapLayer};

pub const KBDSIZE_LAYERS: usize = 3;

#[allow(dead_code)]
#[rustfmt::skip]
const LAYER_0: KeyMapLayer = [
    [   TAB,    Q,      W,      E,      R,      T,      NO_SW,          NO_SW,  Y,      U,      I,      O,      P,      BACKS,  ],
    [   LCTRL,  A,      S,      D,      F,      G,      NO_SW,          NO_SW,  H,      J,      K,      L,      HYPN,   ENTER,  ],
    [   LSFT,   Z,      X,      C,      V,      B,      NO_SW,          NO_SW,  N,      M,      COMMA,  DOT,    SLASH,  RSFT,   ],
    [   NO_SW,  NO_SW,   LGUI,   LALT,   LANG2,    L2,    LCTRL,          RSFT,  L1,  LANG1,  RALT,   RGUI,   NO_SW,   NO_SW,  ],
    [   NO_SW,  NO_SW,  NO_SW,  NO_SW,  DEL,  SPACE,   TAB,             BACKS,   ENTER,   L2,  NO_SW,  NO_SW,  NO_SW,  NO_SW,  ]
];

#[allow(dead_code)]
#[rustfmt::skip]
const LAYER_1: KeyMapLayer = [
    [   ESC,    EXCL,   AT_M,   HASH,   DOLL,   L_CB,   NO_SW,          NO_SW,  R_CB,   COLON,  UP,     BC_SL,  PIPE,   BACKS,  ],
    [   LCTRL,  PCNT,   HAT,    AND,    STAR,   L_RB,   NO_SW,          NO_SW,  R_RB,   LEFT,   DOWN,   RIGHT,  U_SCO,  ENTER,  ],
    [   LSFT,   TILDE,  BK_QT,  D_QT,   S_QT,   L_SB,   NO_SW,          NO_SW,  R_SB,   S_CLN,  L_AB,   R_AB,   QST_M,  RSFT,   ],
    [   NO_SW,  NO_SW,   LGUI,   LALT,   LANG2,    L2,    LCTRL,          RSFT,  L1,  LANG1,  RALT,   RGUI,   NO_SW,   NO_SW,  ],
    [   NO_SW,  NO_SW,  NO_SW,  NO_SW,  DEL,  SPACE,   TAB,             BACKS,   ENTER,   L2,  NO_SW,  NO_SW,  NO_SW,  NO_SW,  ],
];

#[allow(dead_code)]
#[rustfmt::skip]
const LAYER_2: KeyMapLayer = [
    [   TAB,    F1,     F2,     F3,     F4,     NUM_L,  NO_SW,          NO_SW,  PD_PS,  PD_7,   PD_8,   PD_9,   PD_ST,  BACKS,  ],
    [   LCTRL,  F5,     F6,     F7,     F8,     CPS_L,  NO_SW,          NO_SW,  PD_DT,  PD_4,   PD_5,   PD_6,   EQUAL,  ENTER,  ],
    [   LSFT,   F9,     F10,    F11,    F12,    SCR_L,  NO_SW,          NO_SW,  PD_0,   PD_1,   PD_2,   PD_3,   SLASH,  RSFT,   ],
    [   NO_SW,  NO_SW,   LGUI,   LALT,   LANG2,    L2,    LCTRL,          RSFT,  L1,  LANG1,  RALT,   RGUI,   NO_SW,   NO_SW,  ],
    [   NO_SW,  NO_SW,  NO_SW,  NO_SW,  DEL,  SPACE,   TAB,             BACKS,   ENTER,   L2,  NO_SW,  NO_SW,  NO_SW,  NO_SW,  ],
];

pub const KEYMAP: KeyMap = [LAYER_0, LAYER_1, LAYER_2];
