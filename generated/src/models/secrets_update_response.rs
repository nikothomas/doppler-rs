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
/// SecretsUpdateResponse
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretsUpdateResponse {
    #[serde(rename = "secrets", skip_serializing_if = "Option::is_none")]
    pub secrets: Option<models::UpdatedSecretsMap >,
}

impl SecretsUpdateResponse {
    pub fn new() -> SecretsUpdateResponse {
        SecretsUpdateResponse {
            secrets: None,
        }
    }
}
 