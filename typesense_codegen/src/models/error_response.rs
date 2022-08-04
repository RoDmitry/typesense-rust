/*
 * Typesense API
 *
 * An open source search engine for building delightful search experiences.
 *
 * The version of the OpenAPI document: 0.23.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl ErrorResponse {
    pub fn new() -> ErrorResponse {
        ErrorResponse { message: None }
    }
}
