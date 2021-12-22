/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse40390 {
    /// Specifies the version of the API that incorporates this endpoint.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Defines the ID of the request. The `requestId` is generated by Crypto APIs and it's unique for every request.
    #[serde(rename = "requestId")]
    pub request_id: String,
    /// In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user.
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "error")]
    pub error: Box<crate::models::GetAssetDetailsByAssetIde403>,
}

impl InlineResponse40390 {
    pub fn new(api_version: String, request_id: String, error: crate::models::GetAssetDetailsByAssetIde403) -> InlineResponse40390 {
        InlineResponse40390 {
            api_version,
            request_id,
            context: None,
            error: Box::new(error),
        }
    }
}


