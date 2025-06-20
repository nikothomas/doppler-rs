/*
 * Core
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0-oas3.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/*
 * Core
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0-oas3.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, NaiveDate, Utc};
/// OriginalSecretValueType : The valueType you expect the secret to have before `valueType` is applied. If specified, the request will only be processed if the provided valueType matches what's found in Doppler.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OriginalSecretValueType {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String >,
}

impl OriginalSecretValueType {
    pub fn new() -> OriginalSecretValueType {
        OriginalSecretValueType {
            r#type: None,
        }
    }
}
/// String enumeration
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "json5")]
    Json5,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "decimal")]
    Decimal,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "uuidv4")]
    Uuidv4,
    #[serde(rename = "cuid2")]
    Cuid2,
    #[serde(rename = "ulid")]
    Ulid,
    #[serde(rename = "datetime8601")]
    Datetime8601,
    #[serde(rename = "date8601")]
    Date8601,
    #[serde(rename = "yaml")]
    Yaml,
}

impl Default for Type {
    fn default() -> Type {
        Self::String
    }
}
 