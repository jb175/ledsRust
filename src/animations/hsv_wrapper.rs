use smart_leds::hsv::Hsv;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Serialize, Deserialize)]
pub struct HsvWrapper {
    pub hue: u8,
    pub sat: u8,
    pub val: u8,
}

impl From<HsvWrapper> for Hsv {
    fn from(wrapper: HsvWrapper) -> Self {
        Hsv {
            hue: wrapper.hue,
            sat: wrapper.sat,
            val: wrapper.val,
        }
    }
}

impl From<Hsv> for HsvWrapper {
    fn from(hsv: Hsv) -> Self {
        HsvWrapper {
            hue: hsv.hue,
            sat: hsv.sat,
            val: hsv.val,
        }
    }
}

impl Deref for HsvWrapper {
    type Target = Hsv;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const HsvWrapper as *const Hsv) }
    }
}