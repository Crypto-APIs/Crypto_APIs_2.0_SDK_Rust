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
pub struct NewConfirmedTokensTransactionsE403 {
    /// Specifies an error code, e.g. error 404.
    #[serde(rename = "code")]
    pub code: String,
    /// Specifies the message of the error, i.e. why the error was returned, e.g. error 404 stands for “not found”.
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<crate::models::BannedIpAddressDetails>>,
}

impl NewConfirmedTokensTransactionsE403 {
    pub fn new(code: String, message: String) -> NewConfirmedTokensTransactionsE403 {
        NewConfirmedTokensTransactionsE403 {
            code,
            message,
            details: None,
        }
    }
}


