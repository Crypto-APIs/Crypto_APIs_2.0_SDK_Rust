/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// TokensForwardingFailDataItem : Defines an `item` as one result.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokensForwardingFailDataItem {
    /// Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc.
    #[serde(rename = "blockchain")]
    pub blockchain: String,
    /// Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\", \"rinkeby\" are test networks.
    #[serde(rename = "network")]
    pub network: String,
    /// Represents the hash of the address that provides the tokens.
    #[serde(rename = "fromAddress")]
    pub from_address: String,
    /// Represents the hash of the address to forward the tokens to.
    #[serde(rename = "toAddress")]
    pub to_address: String,
    /// Defines the unique Transaction ID that triggered the token forwarding.
    #[serde(rename = "triggerTransactionId")]
    pub trigger_transaction_id: String,
    /// Represents the error code received for the failed token forwarding.
    #[serde(rename = "errorCode")]
    pub error_code: ErrorCode,
    /// Represents the error message received for the failed token forwarding.
    #[serde(rename = "errorMessage")]
    pub error_message: String,
}

impl TokensForwardingFailDataItem {
    /// Defines an `item` as one result.
    pub fn new(blockchain: String, network: String, from_address: String, to_address: String, trigger_transaction_id: String, error_code: ErrorCode, error_message: String) -> TokensForwardingFailDataItem {
        TokensForwardingFailDataItem {
            blockchain,
            network,
            from_address,
            to_address,
            trigger_transaction_id,
            error_code,
            error_message,
        }
    }
}

/// Represents the error code received for the failed token forwarding.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ErrorCode {
    #[serde(rename = "NOT_ENOUGH_CREDITS")]
    NOTENOUGHCREDITS,
    #[serde(rename = "FEE_ADDRESS_OUT_OF_FUNDS")]
    FEEADDRESSOUTOFFUNDS,
    #[serde(rename = "WRONG_ADDRESS_CREDENTIALS")]
    WRONGADDRESSCREDENTIALS,
}

