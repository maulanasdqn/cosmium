use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaDevice {
    pub kind: MediaDeviceKind,
    pub label: String,
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[serde(rename = "groupId")]
    pub group_id: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MediaDeviceKind {
    AudioInput,
    AudioOutput,
    VideoInput,
}
