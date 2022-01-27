// trust_yaml_proofs_mod.rs

use serde_derive::{Deserialize, Serialize};
use unwrap::unwrap;

#[derive(Serialize, Deserialize, Clone)]
pub struct TrustYaml {
    pub ids: Vec<IdYaml>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct IdYaml {
    pub id: String,
    pub url: Option<String>,
}

impl TrustYaml {
    /// from str
    #[allow(dead_code)]
    pub fn from_str(yaml: &str) -> TrustYaml {
        let trust_yaml: TrustYaml = unwrap!(serde_yaml::from_str(yaml));
        trust_yaml
    }
}
