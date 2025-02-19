use serde::{Deserialize, Serialize};
use smart_leds::hsv::hsv2rgb;
use smart_leds::SmartLedsWrite;
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;
use std::time::Duration;
use std::thread;

use super::animation::SerializableAnimation;
use super::Animation;
use crate::animations::hsv_wrapper::Hsv;

#[derive(Debug, Serialize, Deserialize)]
pub struct RainbowAnimation {
    start_color: Hsv,
    duration: Duration,
}

impl RainbowAnimation {
    pub fn new(duration: Duration) -> Self {
        Self {
            start_color : Hsv::new(0, 255, 255),
            duration }
    }
}

impl Animation for RainbowAnimation {
    fn run(&self, ws2812: &mut Ws2812Esp32Rmt) {
        let pixels = std::iter::repeat(hsv2rgb(*self.start_color)).take(60);
        ws2812.write(pixels).unwrap();
        thread::sleep(self.duration);
        // let pixels = std::iter::repeat(hsv2rgb(Hsv { hue: 0, sat: 0, val: 0 })).take(60);
        // ws2812.write(pixels).unwrap();
        // thread::sleep(self.duration);
    }
}