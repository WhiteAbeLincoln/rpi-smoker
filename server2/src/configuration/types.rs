use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct AssetDef {
    #[serde(skip)]
    name: String,
    plugin: String,
    #[serde(default = "HashMap::new")]
    config: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct StationDef {
    #[serde(skip)]
    name: String,
    assets: Vec<AssetDef>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct AeroCfg {
    #[serde(default = "HashMap::new")]
    stations: HashMap<String, StationDef>,
}
