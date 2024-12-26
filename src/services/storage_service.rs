use std::time::Duration;

use esp_idf_svc::nvs::{EspNvs, NvsDefault};

use crate::animations::{Animation, HsvWrapper, PulseAnimation};

pub struct StorageService {
    storage: EspNvs<NvsDefault>,
}

impl StorageService {
    pub fn new(storage : EspNvs<NvsDefault>) -> Self {
        Self { storage }
    }
    
    pub fn load_animation_list(&self) -> anyhow::Result<Vec<Box<dyn Animation>>> {
        let mut animation_list: Vec<Box<dyn Animation>> = Vec::new();
        let mut buf = [0u8; 1024]; // Define a buffer of sufficient size

        // Load animations from NVS
        match self.storage.get_raw("animation_list", &mut buf) {
            Ok(Some(data)) => {
                let json = std::str::from_utf8(data)?;
                let animations = serde_json::from_str::<Vec<(String, HsvWrapper, u64)>>(json)?;
                for (name, color, duration) in animations {
                    if name == "PulseAnimation" {
                        animation_list.push(Box::new(PulseAnimation::new(color, Duration::from_millis(duration))));
                    }
                }
            }
            Ok(None) => {
                log::info!("No animation list found in NVS.");
            }
            Err(e) => {
                log::error!("Failed to load animation list from NVS: {:?}", e);
            }
        }

        Ok(animation_list)
    }

    pub fn save_animation_list(&mut self, animation_list: &Vec<Box<dyn Animation>>) -> anyhow::Result<()> {
        let mut animations = Vec::new();

        for animation in animation_list {
            if let Some(pulse_animation) = animation.as_any().downcast_ref::<PulseAnimation>() {
                let info = animation.get_info();
                animations.push(info);
            }
        }

        let json = serde_json::to_string(&animations)?;
        self.storage.set_raw("animation_list", json.as_bytes())?;

        Ok(())
    }
}
