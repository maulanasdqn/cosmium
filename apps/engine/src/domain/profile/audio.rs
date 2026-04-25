use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audio {
    pub sample_rate: u32,
    pub base_latency: f64,
    pub output_latency: f64,
    #[serde(default = "default_channel_count")]
    pub max_channel_count: u32,
}

const fn default_channel_count() -> u32 {
    2
}
