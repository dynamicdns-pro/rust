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
pub struct Update200ResponseAnyOf {
    #[serde(rename = "message")]
    pub message: String,
}

impl Update200ResponseAnyOf {
    pub fn new(message: String) -> Update200ResponseAnyOf {
        Update200ResponseAnyOf {
            message,
        }
    }
}

