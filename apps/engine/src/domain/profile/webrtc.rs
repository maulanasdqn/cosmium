use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebRtc {
    pub ip_handling_policy: IpHandlingPolicy,
    #[serde(default)]
    pub stun_servers: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IpHandlingPolicy {
    Default,
    DefaultPublicInterfaceOnly,
    DefaultPublicAndPrivateInterfaces,
    DisableNonProxiedUdp,
}
