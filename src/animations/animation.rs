use serde::{Deserialize, Serialize};
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;
use std::time::Duration;

use super::Hsv;

pub trait Animation {
    fn run(&self, ws2812: &mut Ws2812Esp32Rmt);
}

pub trait SerializableAnimation: Animation + Serialize + for<'a> Deserialize<'a> {
    fn run(&self, ws2812: &mut Ws2812Esp32Rmt);
}

impl<T> SerializableAnimation for T where T: Animation + Serialize + for<'de> Deserialize<'de> {
    fn run(&self, ws2812: &mut Ws2812Esp32Rmt) {
        T::run(self, ws2812);
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub enum AnimationType {
    Pulse,
    Rainbow,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AnimationParameters {
    Pulse(Hsv, Duration),
    Rainbow(Duration),
}