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
/// AuthMeResponse
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthMeResponse {
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String >,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String >,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc> >,
    #[serde(rename = "last_seen_at", skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<chrono::DateTime<chrono::Utc> >,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String >,
    #[serde(rename = "token_preview", skip_serializing_if = "Option::is_none")]
    pub token_preview: Option<String >,
    #[serde(rename = "workplace", skip_serializing_if = "Option::is_none")]
    pub workplace: Option<models::AuthWorkplace >,
}

impl AuthMeResponse {
    pub fn new() -> AuthMeResponse {
        AuthMeResponse {
            slug: None,
            name: None,
            created_at: None,
            last_seen_at: None,
            r#type: None,
            token_preview: None,
            workplace: None,
        }
    }
}
 