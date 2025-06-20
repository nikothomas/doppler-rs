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
/// ConfigLogsListResponse
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigLogsListResponse {
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i32 >,
    #[serde(rename = "logs", skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<models::ConfigLog> >,
}

impl ConfigLogsListResponse {
    pub fn new() -> ConfigLogsListResponse {
        ConfigLogsListResponse {
            page: Some(0),
            logs: None,
        }
    }
}
 