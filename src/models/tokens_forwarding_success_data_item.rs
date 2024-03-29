/*
 * CryptoAPIs
 *
 * Crypto APIs is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2021-03-20
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// TokensForwardingSuccessDataItem : Defines an `item` as one result.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokensForwardingSuccessDataItem {
    /// Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc.
    #[serde(rename = "blockchain")]
    pub blockchain: String,
    /// Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks.
    #[serde(rename = "network")]
    pub network: String,
    /// Represents the hash of the address that provides the tokens.
    #[serde(rename = "fromAddress")]
    pub from_address: String,
    /// Represents the hash of the address to forward the tokens to.
    #[serde(rename = "toAddress")]
    pub to_address: String,
    /// Represents the amount of the fee spent for the tokens to be forwarded.
    #[serde(rename = "spentFeesAmount")]
    pub spent_fees_amount: String,
    /// Represents the unit of the fee spent for the tokens to be forwarded, e.g. BTC.
    #[serde(rename = "spentFeesUnit")]
    pub spent_fees_unit: String,
    /// Defines the unique Transaction ID that triggered the token forwarding.
    #[serde(rename = "triggerTransactionId")]
    pub trigger_transaction_id: String,
    /// Defines the unique Transaction ID that forwarded the tokens.
    #[serde(rename = "forwardingTransactionId")]
    pub forwarding_transaction_id: String,
    /// Defines the type of token sent with the transaction, e.g. ERC 20.
    #[serde(rename = "tokenType")]
    pub token_type: TokenType,
    #[serde(rename = "token")]
    pub token: Box<crate::models::TokensForwardingSuccessToken>,
}

impl TokensForwardingSuccessDataItem {
    /// Defines an `item` as one result.
    pub fn new(blockchain: String, network: String, from_address: String, to_address: String, spent_fees_amount: String, spent_fees_unit: String, trigger_transaction_id: String, forwarding_transaction_id: String, token_type: TokenType, token: crate::models::TokensForwardingSuccessToken) -> TokensForwardingSuccessDataItem {
        TokensForwardingSuccessDataItem {
            blockchain,
            network,
            from_address,
            to_address,
            spent_fees_amount,
            spent_fees_unit,
            trigger_transaction_id,
            forwarding_transaction_id,
            token_type,
            token: Box::new(token),
        }
    }
}

/// Defines the type of token sent with the transaction, e.g. ERC 20.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TokenType {
    #[serde(rename = "ERC-20")]
    ERC20,
    #[serde(rename = "ERC-721")]
    ERC721,
    #[serde(rename = "OMNI")]
    OMNI,
}

