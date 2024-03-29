/*
 * CryptoAPIs
 *
 * Crypto APIs is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2021-03-20
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// BroadcastTransactionSuccessDataItem : Defines an `item` as one result.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BroadcastTransactionSuccessDataItem {
    /// Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc.
    #[serde(rename = "blockchain")]
    pub blockchain: String,
    /// Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\", \"rinkeby\" are test networks.
    #[serde(rename = "network")]
    pub network: String,
    /// Defines the unique ID of the specific transaction, i.e. its identification number.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
}

impl BroadcastTransactionSuccessDataItem {
    /// Defines an `item` as one result.
    pub fn new(blockchain: String, network: String, transaction_id: String) -> BroadcastTransactionSuccessDataItem {
        BroadcastTransactionSuccessDataItem {
            blockchain,
            network,
            transaction_id,
        }
    }
}


