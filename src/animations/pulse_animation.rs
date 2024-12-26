use smart_leds::{SmartLedsWrite, hsv::{Hsv, hsv2rgb}};
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;
use std::{any::Any, time::Duration};
use std::thread;

use super::Animation;
use crate::animations::hsv_wrapper::HsvWrapper;

pub struct PulseAnimation {
    color: HsvWrapper,
    duration: Duration,
}

impl PulseAnimation {
    pub fn new(color: HsvWrapper, duration: Duration) -> Self {
        Self { color, duration }
    }
}

impl Animation for PulseAnimation {
    fn run(&self, ws2812: &mut Ws2812Esp32Rmt) {
        let pixels = std::iter::repeat(hsv2rgb(*self.color)).take(60);
        ws2812.write(pixels).unwrap();
        thread::sleep(self.duration);
        // let pixels = std::iter::repeat(hsv2rgb(Hsv { hue: 0, sat: 0, val: 0 })).take(60);
        // ws2812.write(pixels).unwrap();
        // thread::sleep(self.duration);
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_info(&self) -> (String, HsvWrapper, u64) {
        ("PulseAnimation".to_string(), self.color.clone().into(), self.duration.as_millis() as u64)
    }
}