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
pub struct SearchSynonym {
    /// For 1-way synonyms, indicates the root word that words in the `synonyms` parameter map to.
    #[serde(rename = "root", skip_serializing_if = "Option::is_none")]
    pub root: Option<String>,
    /// Array of words that should be considered as synonyms.
    #[serde(rename = "synonyms")]
    pub synonyms: Vec<String>,
    #[serde(rename = "id")]
    pub id: String,
}

impl SearchSynonym {
    pub fn new(synonyms: Vec<String>, id: String) -> SearchSynonym {
        SearchSynonym {
            root: None,
            synonyms,
            id,
        }
    }
}
