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
/// UpdatedStripeSecretData
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdatedStripeSecretData {
    #[serde(rename = "raw", skip_serializing_if = "Option::is_none")]
    pub raw: Option<String >,
    #[serde(rename = "computed", skip_serializing_if = "Option::is_none")]
    pub computed: Option<String >,
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String >,
}

impl UpdatedStripeSecretData {
    pub fn new() -> UpdatedStripeSecretData {
        UpdatedStripeSecretData {
            raw: None,
            computed: None,
            note: None,
        }
    }
}
 