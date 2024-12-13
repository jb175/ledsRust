use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;
use std::time::Duration;

pub trait Animation {
    fn run(&self, ws2812: &mut Ws2812Esp32Rmt, duration: Duration);
}