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
pub struct BroadcastLocallySignedTransactionRbDataItem {
    /// Represents the Secret Key value provided by the customer. This field is used for security purposes during the callback notification, in order to prove the sender of the callback as Crypto APIs. For more information please see our [Documentation](https://developers.cryptoapis.io/technical-documentation/general-information/callbacks#callback-security).
    #[serde(rename = "callbackSecretKey", skip_serializing_if = "Option::is_none")]
    pub callback_secret_key: Option<String>,
    /// Represents the URL that is set by the customer where the callback will be received at. The callback notification will be received only if and when the event occurs. `We support ONLY httpS type of protocol`.
    #[serde(rename = "callbackUrl", skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    /// Represents the signed transaction's specific hex.
    #[serde(rename = "signedTransactionHex")]
    pub signed_transaction_hex: String,
}

impl BroadcastLocallySignedTransactionRbDataItem {
    pub fn new(signed_transaction_hex: String) -> BroadcastLocallySignedTransactionRbDataItem {
        BroadcastLocallySignedTransactionRbDataItem {
            callback_secret_key: None,
            callback_url: None,
            signed_transaction_hex,
        }
    }
}


