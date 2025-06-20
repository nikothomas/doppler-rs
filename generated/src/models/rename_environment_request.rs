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
/// RenameEnvironmentRequest
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenameEnvironmentRequest {
    /// Desired name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String >,
    /// Desired slug
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String >,
    /// Whether or not to enable personal configs for the environment
    #[serde(rename = "personal_configs", skip_serializing_if = "Option::is_none")]
    pub personal_configs: Option<bool >,
}

impl RenameEnvironmentRequest {
    pub fn new() -> RenameEnvironmentRequest {
        RenameEnvironmentRequest {
            name: None,
            slug: None,
            personal_configs: None,
        }
    }
}
 