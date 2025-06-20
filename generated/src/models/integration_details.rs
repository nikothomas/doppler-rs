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
/// IntegrationDetails
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationDetails {
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String >,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String >,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String >,
}

impl IntegrationDetails {
    pub fn new() -> IntegrationDetails {
        IntegrationDetails {
            slug: None,
            name: None,
            r#type: None,
        }
    }
}
 