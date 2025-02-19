use serde::{Deserialize, Serialize};
use smart_leds::{SmartLedsWrite, hsv::hsv2rgb};
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;
use std::time::Duration;
use std::thread;

use crate::animations::hsv_wrapper::Hsv;

use super::animation::SerializableAnimation;

#[derive(Debug, Serialize, Deserialize)]
pub struct PulseAnimation {
    color: Hsv,
    duration: Duration,
}

impl PulseAnimation {
    pub fn new(color: Hsv, duration: Duration) -> Self {
        Self { color, duration }
    }
}


impl SerializableAnimation for PulseAnimation {
    fn run(&self, ws2812: &mut Ws2812Esp32Rmt) {
        let pixels = std::iter::repeat(hsv2rgb(*self.color)).take(60);
        ws2812.write(pixels).unwrap();
        thread::sleep(self.duration);
        // let pixels = std::iter::repeat(hsv2rgb(Hsv { hue: 0, sat: 0, val: 0 })).take(60);
        // ws2812.write(pixels).unwrap();
        // thread::sleep(self.duration);
    }
}