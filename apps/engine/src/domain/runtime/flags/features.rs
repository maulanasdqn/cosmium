use crate::domain::profile::IpHandlingPolicy;

pub(super) fn disable_features_list() -> &'static str {
    "Translate,\
     InterestFeedContentSuggestions,\
     PrivacySandboxAdsAPIs,\
     OptimizationHints,\
     MediaRouter,\
     DialMediaRouteProvider,\
     AcceptCHFrame,\
     AutofillServerCommunication,\
     CertificateTransparencyComponentUpdater,\
     GlobalMediaControls,\
     ImprovedCookieControls,\
     LazyFrameLoading,\
     PreloadMediaEngagementData,\
     MediaEngagementBypassAutoplayPolicies"
}

pub(super) fn webrtc_flags(policy: &IpHandlingPolicy) -> Vec<String> {
    let value = match policy {
        IpHandlingPolicy::Default => "default",
        IpHandlingPolicy::DefaultPublicInterfaceOnly => "default_public_interface_only",
        IpHandlingPolicy::DefaultPublicAndPrivateInterfaces => {
            "default_public_and_private_interfaces"
        }
        IpHandlingPolicy::DisableNonProxiedUdp => "disable_non_proxied_udp",
    };
    vec![format!("--force-webrtc-ip-handling-policy={value}")]
}
