/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// TransactionRequestFailDataItem : Defines an `item` as one result.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransactionRequestFailDataItem {
    /// Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc.
    #[serde(rename = "blockchain")]
    pub blockchain: String,
    /// Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\", \"rinkeby\" are test networks.
    #[serde(rename = "network")]
    pub network: String,
    /// The required number of approvals needed to approve the transaction.
    #[serde(rename = "requiredApprovals")]
    pub required_approvals: i32,
    /// The required number of rejections needed to reject the transaction.
    #[serde(rename = "requiredRejections")]
    pub required_rejections: i32,
    /// The current number of approvals given for the transaction.
    #[serde(rename = "currentApprovals")]
    pub current_approvals: i32,
    /// The current number of rejections given for the transaction.
    #[serde(rename = "currentRejections")]
    pub current_rejections: i32,
    /// Represents the error message received for the transaction.
    #[serde(rename = "errorMessage")]
    pub error_message: String,
}

impl TransactionRequestFailDataItem {
    /// Defines an `item` as one result.
    pub fn new(blockchain: String, network: String, required_approvals: i32, required_rejections: i32, current_approvals: i32, current_rejections: i32, error_message: String) -> TransactionRequestFailDataItem {
        TransactionRequestFailDataItem {
            blockchain,
            network,
            required_approvals,
            required_rejections,
            current_approvals,
            current_rejections,
            error_message,
        }
    }
}


