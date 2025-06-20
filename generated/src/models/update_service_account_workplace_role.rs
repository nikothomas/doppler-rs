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
/// UpdateServiceAccountWorkplaceRole : You may provide an identifier OR permissions, but not both
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateServiceAccountWorkplaceRole {
    /// Identifier of an existing workplace role
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String >,
    /// Workplace permissions to grant
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String> >,
}

impl UpdateServiceAccountWorkplaceRole {
    pub fn new() -> UpdateServiceAccountWorkplaceRole {
        UpdateServiceAccountWorkplaceRole {
            identifier: None,
            permissions: None,
        }
    }
}
 