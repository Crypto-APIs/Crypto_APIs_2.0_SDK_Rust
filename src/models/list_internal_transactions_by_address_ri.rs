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
pub struct ListInternalTransactionsByAddressRi {
    /// Defines the specific amount of the transaction.
    #[serde(rename = "amount")]
    pub amount: String,
    /// Represents the hash of the block where this transaction was mined/confirmed for first time. The hash is defined as a cryptographic digital fingerprint made by hashing the block header twice through the SHA256 algorithm.
    #[serde(rename = "minedInBlockHash")]
    pub mined_in_block_hash: String,
    /// Represents the hight of the block where this transaction was mined/confirmed for first time. The height is defined as the number of blocks in the blockchain preceding this specific block.
    #[serde(rename = "minedInBlockHeight")]
    pub mined_in_block_height: i32,
    /// Represents the unique internal transaction ID in regards to the parent transaction (type trace address).
    #[serde(rename = "operationID")]
    pub operation_id: String,
    /// Defines the call type of the internal transaction.
    #[serde(rename = "operationType")]
    pub operation_type: String,
    /// Defines the specific hash of the parent transaction.
    #[serde(rename = "parentHash")]
    pub parent_hash: String,
    /// Represents the recipient address with the respective amount.
    #[serde(rename = "recipient")]
    pub recipient: String,
    /// Represents the sender address with the respective amount.
    #[serde(rename = "sender")]
    pub sender: String,
    #[serde(rename = "timestamp")]
    pub timestamp: i32,
}

impl ListInternalTransactionsByAddressRi {
    pub fn new(amount: String, mined_in_block_hash: String, mined_in_block_height: i32, operation_id: String, operation_type: String, parent_hash: String, recipient: String, sender: String, timestamp: i32) -> ListInternalTransactionsByAddressRi {
        ListInternalTransactionsByAddressRi {
            amount,
            mined_in_block_hash,
            mined_in_block_height,
            operation_id,
            operation_type,
            parent_hash,
            recipient,
            sender,
            timestamp,
        }
    }
}


