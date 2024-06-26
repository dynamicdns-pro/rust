/*
 * Dynamicdns.pro
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateRequest {
    #[serde(rename = "ipv4", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<Option<String>>,
    #[serde(rename = "ipv6", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<Option<String>>,
}

impl UpdateRequest {
    pub fn new() -> UpdateRequest {
        UpdateRequest {
            ipv4: None,
            ipv6: None,
        }
    }
}

