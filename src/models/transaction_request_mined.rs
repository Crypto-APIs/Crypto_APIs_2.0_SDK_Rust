/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransactionRequestMined {
    /// Specifies the version of the API that incorporates this endpoint.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Represents a unique identifier that serves as reference to the specific request which prompts a callback, e.g. Blockchain Events Subscription, Blockchain Automation, etc.
    #[serde(rename = "referenceId")]
    pub reference_id: String,
    /// Specifies a unique ID generated by the system and attached to each callback. It is used by the server to recognize consecutive requests with the same data with the purpose not to perform the same operation twice.
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: String,
    #[serde(rename = "data")]
    pub data: Box<crate::models::TransactionRequestMinedData>,
}

impl TransactionRequestMined {
    pub fn new(api_version: String, reference_id: String, idempotency_key: String, data: crate::models::TransactionRequestMinedData) -> TransactionRequestMined {
        TransactionRequestMined {
            api_version,
            reference_id,
            idempotency_key,
            data: Box::new(data),
        }
    }
}


