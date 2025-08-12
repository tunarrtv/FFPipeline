use std::collections::HashSet;
use crate::model::HardwareAccelerationMode;

pub struct BinaryCapabilities {
    hardware_accelerations: HashSet<String>,
    encoders: HashSet<String>,
    decoder: HashSet<String>,
    filters: HashSet<String>,
    options: HashSet<String>,
}

impl BinaryCapabilities {
    pub fn has_hardware_acceleration(&self, hw: HardwareAccelerationMode) -> bool {
        if hw == HardwareAccelerationMode::Amf {
            return self.encoders.iter().find(|e| e.ends_with("amf")).is_some();
        }

        let accel = match hw {
            HardwareAccelerationMode::Qsv => Some("qsv"), // TODO: Constants
            HardwareAccelerationMode::Cuda => Some("cuda"),
            HardwareAccelerationMode::Vaapi => Some("vaapi"),
            HardwareAccelerationMode::VideoToolbox => Some("videotoolbox"),
            _ => None,
        };

        match accel {
            None => false,
            Some(v) => self.hardware_accelerations.contains(v)
        }
    }

    pub fn has_option(&self, option: &str) -> bool {
        self.options.contains(option)
    }
}