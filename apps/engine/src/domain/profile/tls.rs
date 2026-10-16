use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Tls {
    #[serde(default)]
    pub ciphers: Option<String>,

    #[serde(default)]
    pub sigalgs: Option<Vec<String>>,

    #[serde(default)]
    pub curves: Option<Vec<String>>,

    #[serde(default)]
    pub alpn: Option<Vec<String>>,

    #[serde(default)]
    pub min_version: Option<TlsVersion>,

    #[serde(default)]
    pub max_version: Option<TlsVersion>,

    #[serde(default)]
    pub permute_extensions: Option<bool>,

    #[serde(default)]
    pub grease: Option<bool>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TlsVersion {
    #[serde(rename = "tls1.2")]
    Tls12,
    #[serde(rename = "tls1.3")]
    Tls13,
}

impl Tls {
    pub fn push_flags(&self, f: &mut Vec<String>) {
        if let Some(c) = &self.ciphers {
            f.push(format!("--cosmium-tls-ciphers={c}"));
        }
        if let Some(s) = &self.sigalgs {
            f.push(format!("--cosmium-tls-sigalgs={}", s.join(",")));
        }
        if let Some(g) = &self.curves {
            f.push(format!("--cosmium-tls-curves={}", g.join(",")));
        }
        if let Some(a) = &self.alpn {
            f.push(format!("--cosmium-tls-alpn={}", a.join(",")));
        }
        if let Some(v) = self.min_version {
            f.push(format!("--cosmium-tls-min-version={}", v.as_str()));
        }
        if let Some(v) = self.max_version {
            f.push(format!("--cosmium-tls-max-version={}", v.as_str()));
        }
        if let Some(pe) = self.permute_extensions {
            f.push(format!("--cosmium-tls-permute-extensions={}", u8::from(pe)));
        }
        if let Some(g) = self.grease {
            f.push(format!("--cosmium-tls-grease={}", u8::from(g)));
        }
    }
}

impl TlsVersion {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Tls12 => "tls1.2",
            Self::Tls13 => "tls1.3",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tls;

    #[test]
    fn empty_tls_produces_no_flags() {
        let tls = Tls::default();
        let mut f = Vec::new();
        tls.push_flags(&mut f);
        assert!(f.is_empty());
    }

    #[test]
    fn populated_tls_produces_expected_flags() {
        let tls: Tls = serde_json::from_str(
            r#"{
                "ciphers": "ALL:!aPSK:!3DES",
                "sigalgs": ["ecdsa_secp256r1_sha256","rsa_pss_rsae_sha256"],
                "curves": ["X25519","P-256","P-384"],
                "alpn": ["h2","http/1.1"],
                "min_version": "tls1.2",
                "max_version": "tls1.3",
                "permute_extensions": true,
                "grease": true
            }"#,
        )
        .expect("fixture parses");

        let mut f = Vec::new();
        tls.push_flags(&mut f);
        assert_eq!(f.len(), 8);
        assert_eq!(f[0], "--cosmium-tls-ciphers=ALL:!aPSK:!3DES");
        assert_eq!(
            f[1],
            "--cosmium-tls-sigalgs=ecdsa_secp256r1_sha256,rsa_pss_rsae_sha256"
        );
        assert_eq!(f[2], "--cosmium-tls-curves=X25519,P-256,P-384");
        assert_eq!(f[3], "--cosmium-tls-alpn=h2,http/1.1");
        assert_eq!(f[4], "--cosmium-tls-min-version=tls1.2");
        assert_eq!(f[5], "--cosmium-tls-max-version=tls1.3");
        assert_eq!(f[6], "--cosmium-tls-permute-extensions=1");
        assert_eq!(f[7], "--cosmium-tls-grease=1");
    }

    #[test]
    fn version_round_trips() {
        let tls: Tls = serde_json::from_str(r#"{"min_version":"tls1.3"}"#).expect("parses");
        assert!(matches!(tls.min_version, Some(super::TlsVersion::Tls13)));
    }
}
