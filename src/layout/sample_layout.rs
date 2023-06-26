use crate::keycodes::*;
use crate::{KeyMap, KeyMapLayer};

#[allow(dead_code)]
#[rustfmt::skip]
const LAYER_0: KeyMapLayer = [
    [   EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  NO_SW,          NO_SW,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  ],
    [   EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  NO_SW,          NO_SW,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  ],
    [   EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  NO_SW,          NO_SW,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  ],
    [   NO_SW,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,          EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  NO_SW,  ],
    [   NO_SW,  NO_SW,  NO_SW,  NO_SW,  EMPTY,  EMPTY,  EMPTY,          EMPTY,  EMPTY,  EMPTY,  NO_SW,  NO_SW,  NO_SW,  NO_SW,  ],
];

#[allow(dead_code)]
#[rustfmt::skip]
const LAYER_1: KeyMapLayer = [
    [   EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  NO_SW,          NO_SW,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  ],
    [   EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  NO_SW,          NO_SW,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  ],
    [   EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  NO_SW,          NO_SW,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  ],
    [   NO_SW,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,          EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  NO_SW,  ],
    [   NO_SW,  NO_SW,  NO_SW,  NO_SW,  EMPTY,  EMPTY,  EMPTY,          EMPTY,  EMPTY,  EMPTY,  NO_SW,  NO_SW,  NO_SW,  NO_SW,  ],
];

#[allow(dead_code)]
#[rustfmt::skip]
const LAYER_2: KeyMapLayer = [
    [   EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  NO_SW,          NO_SW,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  ],
    [   EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  NO_SW,          NO_SW,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  ],
    [   EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  NO_SW,          NO_SW,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  ],
    [   NO_SW,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,          EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  NO_SW,  ],
    [   NO_SW,  NO_SW,  NO_SW,  NO_SW,  EMPTY,  EMPTY,  EMPTY,          EMPTY,  EMPTY,  EMPTY,  NO_SW,  NO_SW,  NO_SW,  NO_SW,  ],
];

#[allow(dead_code)]
#[rustfmt::skip]
const LAYER_3: KeyMapLayer = [
    [   EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  NO_SW,          NO_SW,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  ],
    [   EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  NO_SW,          NO_SW,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  ],
    [   EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  NO_SW,          NO_SW,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  ],
    [   NO_SW,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,          EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  EMPTY,  NO_SW,  ],
    [   NO_SW,  NO_SW,  NO_SW,  NO_SW,  EMPTY,  EMPTY,  EMPTY,          EMPTY,  EMPTY,  EMPTY,  NO_SW,  NO_SW,  NO_SW,  NO_SW,  ],
];

pub const KEYMAP: KeyMap = [LAYER_0, LAYER_1, LAYER_2, LAYER_3];
