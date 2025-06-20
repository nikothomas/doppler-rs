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
/// ServiceAccountTokenCreateResponse
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceAccountTokenCreateResponse {
    #[serde(rename = "api_token", skip_serializing_if = "Option::is_none")]
    pub api_token: Option<models::ServiceAccountTokenDetails >,
    #[serde(rename = "api_key", skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String >,
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool >,
}

impl ServiceAccountTokenCreateResponse {
    pub fn new() -> ServiceAccountTokenCreateResponse {
        ServiceAccountTokenCreateResponse {
            api_token: None,
            api_key: None,
            success: Some(true),
        }
    }
}
 