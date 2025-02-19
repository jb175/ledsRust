use std::time::Duration;

use animations::{Animation, Hsv, PulseAnimation};
use esp_idf_hal::prelude::*;
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;
use esp_idf_svc::{log::EspLogger, nvs::{EspDefaultNvsPartition, EspNvs}};

mod animations;

mod services;
use services::{storage_service::StorageService, bluetooth_service::BluetoothService};

use log::{info, LevelFilter};
fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    //initialize logger
    EspLogger::initialize_default();
    EspLogger.set_target_level("*", LevelFilter::Info).unwrap();

    info!("Start LedStrips!");

    info!("Initializing peripherals");
    // Initialize the peripherals
    let peripherals = Peripherals::take().unwrap();
    let led_pin = peripherals.pins.gpio13;
    let channel = peripherals.rmt.channel0;
    let mut ws2812 = Ws2812Esp32Rmt::new(channel, led_pin).unwrap();

    info!("Initializing storage");
    // Initialize NVS
    let nvs = EspDefaultNvsPartition::take()?;

    info!("Initializing services");
    let mut storage_service = StorageService::new(
        EspNvs::new(nvs, "storage", true)?
    );
    // let bluetooth_service = bluetooth_service::setup(
    //     peripherals.modem
    // );

    let bl_service = BluetoothService::new();
    let _ = bl_service.setup(peripherals.modem);

    // Load animation list from NVS
    // let mut animation_list = storage_service.load_animation_list()?;


    // // Create animation list
    let mut animation_list: Vec<Box<dyn Animation>> = Vec::new();
    
    animation_list.push(Box::new(PulseAnimation::new(Hsv { hue: 0, sat: 255, val: 255 }, Duration::from_millis(500))));
    animation_list.push(Box::new(PulseAnimation::new(Hsv { hue: 120, sat: 255, val: 255 }, Duration::from_millis(500))));
    animation_list.push(Box::new(PulseAnimation::new(Hsv { hue: 240, sat: 255, val: 255 }, Duration::from_millis(500))));
    animation_list.push(Box::new(PulseAnimation::new(Hsv { hue: 60, sat: 255, val: 255 }, Duration::from_millis(500))));

    // Save animation list to NVS
    storage_service.save_animation_list(&animation_list)?;


    // Run animations
    loop {
        for animation in &animation_list {
            animation.run(&mut ws2812);
        }
    }
    
}