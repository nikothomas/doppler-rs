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
/// ActivityLogUser
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityLogUser {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String >,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String >,
    #[serde(rename = "profile_image_url", skip_serializing_if = "Option::is_none")]
    pub profile_image_url: Option<String >,
}

impl ActivityLogUser {
    pub fn new() -> ActivityLogUser {
        ActivityLogUser {
            email: None,
            name: None,
            profile_image_url: None,
        }
    }
}
 