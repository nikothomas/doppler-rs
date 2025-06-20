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
/// PolicyTargetsRequest : Describes the which projects, environments, and configs the policy applies to.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyTargetsRequest {
    /// If true, the policy will apply to every config in the workplace.
    #[serde(rename = "allProjects", skip_serializing_if = "Option::is_none")]
    pub all_projects: Option<bool >,
    /// A dictionary where the key is the project name, and the value contains information about what within the project the policy should apply to.
    #[serde(rename = "projects", skip_serializing_if = "Option::is_none")]
    pub projects: Option<String >,
}

impl PolicyTargetsRequest {
    pub fn new() -> PolicyTargetsRequest {
        PolicyTargetsRequest {
            all_projects: None,
            projects: None,
        }
    }
}
 