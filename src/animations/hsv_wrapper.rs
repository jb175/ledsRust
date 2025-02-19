use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Default)]
pub struct Hsv {
    pub hue: u8,
    pub sat: u8,
    pub val: u8,
}

impl Hsv {
    pub fn new(hue: u8, sat: u8, val: u8) -> Self {
        Self { hue, sat, val }
    }

    pub fn hsv2rgb(hsv: Hsv) -> smart_leds::RGB8 {
        smart_leds::hsv::hsv2rgb(hsv.into())
    }
}

impl From<Hsv> for smart_leds::hsv::Hsv {
    fn from(wrapper: Hsv) -> Self {
        smart_leds::hsv::Hsv {
            hue: wrapper.hue,
            sat: wrapper.sat,
            val: wrapper.val,
        }
    }
}

impl From<smart_leds::hsv::Hsv> for Hsv {
    fn from(hsv: smart_leds::hsv::Hsv) -> Self {
        Hsv {
            hue: hsv.hue,
            sat: hsv.sat,
            val: hsv.val,
        }
    }
}

impl Deref for Hsv {
    type Target = smart_leds::hsv::Hsv;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Hsv as *const smart_leds::hsv::Hsv) }
    }
}