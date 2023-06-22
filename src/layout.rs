// キーボード固有の機能をすべてここに入れたい
use crate::bsp::hal::gpio::DynPin;
use crate::bsp::Pins;
use embedded_hal::digital::v2::{InputPin, OutputPin};

use crate::{KeyMap, KeyMapLayer, KeyMapping};

use KeyMapping::*;

// LED pin12
// 右スイッチ　pin19,pin20
// 左スイッチ　pin10,pin11
// [右,なし]
// [なし,左]

// [US配列ライクなJISキーボードにするカスタムキーコードを作った - Qiita](https://qiita.com/koktoh/items/874be0e4d058aae54180)
// windows側はJISキーボードとしているので記号周りに工夫が必用
// mappingにshiftとキーの同時押しも定義しないとダメそう

pub const KBDSIZE_ROWS: usize = 2;
pub const KBDSIZE_COLS: usize = 2;
pub const KBDSIZE_LED: usize = 0;

pub const KBDSIZE_LAYERS: usize = 2;

pub fn set_pins(
    pins: Pins,
) -> (
    [DynPin; KBDSIZE_ROWS],
    [DynPin; KBDSIZE_COLS],
    [DynPin; KBDSIZE_LED],
) {
    let rows = [
        pins.gpio20.into_push_pull_output().into(),
        pins.gpio11.into_push_pull_output().into(),
    ];
    let cols = [
        pins.gpio19.into_pull_up_input().into(),
        pins.gpio10.into_pull_up_input().into(),
    ];
    // let leds = [pins.gpio12.into_push_pull_output().into()];
    let leds = [];
    (rows, cols, leds)
}

// keycode 0x04 : key A
// keycode 0x05 : key B

#[allow(dead_code)]
#[rustfmt::skip]
const LAYER_0: KeyMapLayer = [
    [K(0x06),  Empty,],
    [Empty,  L(1),],
];
#[allow(dead_code)]
#[rustfmt::skip]
const LAYER_1: KeyMapLayer = [
    [K(0x05),  Empty,],
    [Empty,  L(1),],
];

pub const KEYMAP: KeyMap = [LAYER_0, LAYER_1];
