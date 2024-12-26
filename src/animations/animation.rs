use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;
use std::{any::Any, time::Duration};

use crate::animations::hsv_wrapper::HsvWrapper;

pub trait Animation {
    fn run(&self, ws2812: &mut Ws2812Esp32Rmt);
    fn as_any(&self) -> &dyn Any;
    fn get_info(&self) -> (String, HsvWrapper, u64);
}