/*
 * Typesense API
 *
 * An open source search engine for building delightful search experiences.
 *
 * The version of the OpenAPI document: 0.23.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SearchOverridesResponse {
    #[serde(rename = "overrides")]
    pub overrides: Vec<crate::models::SearchOverride>,
}

impl SearchOverridesResponse {
    pub fn new(overrides: Vec<crate::models::SearchOverride>) -> SearchOverridesResponse {
        SearchOverridesResponse { overrides }
    }
}
