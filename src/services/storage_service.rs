use esp_idf_svc::{log::EspLogger, nvs::{EspNvs, NvsDefault}};

use crate::animations::{animation::AnimationParameters, Animation, PulseAnimation, RainbowAnimation};

use log::{error, info, debug, LevelFilter};


pub struct StorageService {
    storage: EspNvs<NvsDefault>,
}

impl StorageService {

    const MAIN_TARGET: &'static str = "storage";

    pub fn new(storage : EspNvs<NvsDefault>) -> Self {
        EspLogger.set_target_level(StorageService::MAIN_TARGET, LevelFilter::max()).unwrap();
        Self { storage }
    }
    
    pub fn load_animation_list(&self) -> anyhow::Result<Vec<Box<dyn Animation>>> {
        debug!(target:StorageService::MAIN_TARGET, "Starting to load animation list from NVS.");
        let mut animation_list: Vec<Box<dyn Animation>> = Vec::new();
        let mut buf = [0u8; 1024]; // Define a buffer of sufficient size

        // Load animations from NVS
        match self.storage.get_raw("animation_list", &mut buf) {
            Ok(Some(data)) => {
                let json = std::str::from_utf8(data)?;
                let animations = serde_json::from_str::<Vec<AnimationParameters>>(json)?;
                for animation in animations {
                    let anim: Box<dyn Animation> = match animation {
                        AnimationParameters::Pulse(color, duration) => {
                            Box::new(PulseAnimation::new(color, duration))
                            
                        }
                        AnimationParameters::Rainbow(duration) => {
                            Box::new(RainbowAnimation::new(duration))
                        }
                    };
                    debug!(target:StorageService::MAIN_TARGET, "New animation loaded:");
                    animation_list.push(anim);
                }
            }
            Ok(None) => {
                info!(target:StorageService::MAIN_TARGET, "No animation list found in NVS.");
            }
            Err(e) => {
                error!(target:StorageService::MAIN_TARGET, "Failed to load animation list from NVS: {:?}", e);
            }
        }

        debug!(target:StorageService::MAIN_TARGET, "Finished loading animation list from NVS.");
        Ok(animation_list)
    }

    pub fn save_animation_list(&mut self, animation_list: &Vec<Box<dyn Animation>>) -> anyhow::Result<()> {
        debug!(target:StorageService::MAIN_TARGET, "Starting to save animation list into NVS.");
        let mut animations = Vec::new();

        for animation in animation_list {
            let info = stringify!(animation);
            debug!(target:StorageService::MAIN_TARGET, "animation saved: {info}");
            animations.push(info);
        }

        let json = serde_json::to_string(&animations)?;
        self.storage.set_raw("animation_list", json.as_bytes())?;

        debug!(target:StorageService::MAIN_TARGET, "Finished saving animation list into NVS.");
        Ok(())
    }
}
