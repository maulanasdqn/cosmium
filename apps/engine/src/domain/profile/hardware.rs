use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hardware {
    pub hardware_concurrency: u32,
    pub device_memory_gb: f32,
    pub max_touch_points: u32,
    #[serde(default)]
    pub battery: Option<Battery>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Battery {
    pub charging: bool,
    pub level: f32,
    pub charging_time_seconds: Option<u64>,
    pub discharging_time_seconds: Option<u64>,
}
