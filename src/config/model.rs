use serde::{Deserialize, Serialize};

use crate::epgstation_api::model::RuleId;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EncodeRule {
    pub encode_mode: String,
    pub rules: Option<Vec<RuleId>>,
    pub no_rule: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub epgstation_url: String,
    pub default_encode_mode: String,
    pub encode_rule: Vec<EncodeRule>,
}
