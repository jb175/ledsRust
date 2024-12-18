use std::{thread, time::Duration};
use esp_idf_hal::{modem::Modem, prelude::*};
use esp_idf_sys::*;
use smart_leds::{SmartLedsWrite, hsv::{Hsv, hsv2rgb}};
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;

mod animations;
use animations::{Animation, PulseAnimation};

mod services;
use services::bluetooth_service;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Start LedStrips!");

    // Initialize the peripherals
    let peripherals = Peripherals::take().unwrap();
    let led_pin = peripherals.pins.gpio13;
    let channel = peripherals.rmt.channel0;
    let mut ws2812 = Ws2812Esp32Rmt::new(channel, led_pin).unwrap();

    // Create animation list
    let mut animation_list = Vec::new();
    animation_list.push(Box::new(PulseAnimation::new(Hsv { hue: 0, sat: 255, val: 255 }, Duration::from_millis(500))));
    animation_list.push(Box::new(PulseAnimation::new(Hsv { hue: 120, sat: 255, val: 255 }, Duration::from_millis(500))));
    animation_list.push(Box::new(PulseAnimation::new(Hsv { hue: 240, sat: 255, val: 255 }, Duration::from_millis(500))));

    let bluetooth_service = bluetooth_service::setup();

    log::info!("Continue LedStrips!");

    // Run animations
    loop {
        for animation in &animation_list {
            animation.run(&mut ws2812, Duration::from_secs(1));
        }
    }
    
}
