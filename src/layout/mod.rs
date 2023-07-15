// キーボード固有の機能をすべてここに入れたい
use crate::bsp::hal::gpio::DynPin;
use crate::bsp::Pins;

pub const KBDSIZE_ROWS: usize = 5;
pub const KBDSIZE_COLS: usize = 14;
pub const KBDSIZE_LED: usize = 0;

// レイアウトファイルの指定
// mod sample_layout;
// pub use sample_layout::{KBDSIZE_LAYERS, KEYMAP};

// mod ctsc;
// pub use ctsc::{KBDSIZE_LAYERS, KEYMAP};

mod test_map;
pub use test_map::{KBDSIZE_LAYERS, KEYMAP};

pub fn set_pins(
    pins: Pins,
) -> (
    [DynPin; KBDSIZE_ROWS],
    [DynPin; KBDSIZE_COLS],
    [DynPin; KBDSIZE_LED],
) {
    let rows = [
        pins.gpio13.into_push_pull_output().into(),
        pins.gpio14.into_push_pull_output().into(),
        pins.gpio15.into_push_pull_output().into(),
        pins.gpio17.into_push_pull_output().into(),
        pins.gpio16.into_push_pull_output().into(),
    ];
    let cols = [
        // 左手側
        pins.gpio6.into_pull_up_input().into(),
        pins.gpio7.into_pull_up_input().into(),
        pins.gpio8.into_pull_up_input().into(),
        pins.gpio9.into_pull_up_input().into(),
        pins.gpio10.into_pull_up_input().into(),
        pins.gpio11.into_pull_up_input().into(),
        pins.gpio12.into_pull_up_input().into(),
        // 右手側
        pins.gpio18.into_pull_up_input().into(),
        pins.gpio19.into_pull_up_input().into(),
        pins.gpio20.into_pull_up_input().into(),
        pins.gpio21.into_pull_up_input().into(),
        pins.gpio22.into_pull_up_input().into(),
        pins.gpio23.into_pull_up_input().into(),
        pins.gpio24.into_pull_up_input().into(),
    ];
    // let leds = [pins.gpio12.into_push_pull_output().into()];
    let leds = [];
    (rows, cols, leds)
}
