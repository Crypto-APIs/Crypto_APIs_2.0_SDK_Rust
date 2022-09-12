/*
 * CryptoAPIs
 *
 * Crypto APIs is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2021-03-20
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetExchangeRateByAssetSymbolsE422 {
    /// Specifies an error code, e.g. error 404.
    #[serde(rename = "code")]
    pub code: String,
    /// Specifies the message of the error, i.e. why the error was returned, e.g. error 404 stands for “not found”.
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<crate::models::BannedIpAddressDetails>>,
}

impl GetExchangeRateByAssetSymbolsE422 {
    pub fn new(code: String, message: String) -> GetExchangeRateByAssetSymbolsE422 {
        GetExchangeRateByAssetSymbolsE422 {
            code,
            message,
            details: None,
        }
    }
}


