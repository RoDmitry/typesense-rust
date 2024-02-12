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
pub struct MultiSearchCollectionParametersAllOf {
    /// The collection to search in.
    #[serde(rename = "collection")]
    pub collection: String,
}

impl MultiSearchCollectionParametersAllOf {
    pub fn new(collection: String) -> MultiSearchCollectionParametersAllOf {
        MultiSearchCollectionParametersAllOf { collection }
    }
}
