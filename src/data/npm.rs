use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::version::SemanticVersion;

#[derive(Serialize, Deserialize, Debug)]
pub struct NpmPackage
{
    pub name: String,

    #[serde(alias = "version")]
    version_str: String,

    #[serde(skip)]
    pub version: SemanticVersion,
}

impl NpmPackage
{
    pub fn from_json(text: &str) -> Self
    {
        let mut parsed = serde_json::from_str::<Self>(text).unwrap();
        parsed.version = SemanticVersion::from(parsed.version_str.as_str());

        parsed
    }

    pub fn from_json_val(val: &Value) -> Self
    {
        let mut parsed = serde_json::from_value::<Self>(val.clone()).unwrap();
        parsed.version = SemanticVersion::from(parsed.version_str.as_str());

        parsed
    }
}
