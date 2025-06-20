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
/// UpdateIntegrationRequest
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateIntegrationRequest {
    /// The new name of the integration
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String >,
    /// The new authentication data for the integration
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String >,
}

impl UpdateIntegrationRequest {
    pub fn new() -> UpdateIntegrationRequest {
        UpdateIntegrationRequest {
            name: None,
            data: None,
        }
    }
}
 