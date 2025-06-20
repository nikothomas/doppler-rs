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
/// UpdateServiceAccountRequest
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateServiceAccountRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String >,
    #[serde(rename = "workplace_role", skip_serializing_if = "Option::is_none")]
    pub workplace_role: Option<models::UpdateServiceAccountWorkplaceRole >,
}

impl UpdateServiceAccountRequest {
    pub fn new() -> UpdateServiceAccountRequest {
        UpdateServiceAccountRequest {
            name: None,
            workplace_role: None,
        }
    }
}
 