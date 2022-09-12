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
pub struct GetTransactionRequestDetailsRi {
    /// Defines an optional note for additional details.
    #[serde(rename = "additionalDetails")]
    pub additional_details: String,
    /// Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc.
    #[serde(rename = "blockchain")]
    pub blockchain: Blockchain,
    /// Defines the priority for the fee, if it is \"slow\", \"standard\" or \"fast\".
    #[serde(rename = "feePriority")]
    pub fee_priority: FeePriority,
    /// Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks.
    #[serde(rename = "network")]
    pub network: Network,
    /// Represents a list of recipient addresses with the respective amounts. In account-based protocols like Ethereum there is only one address in this list.
    #[serde(rename = "recipients")]
    pub recipients: Vec<crate::models::GetTransactionRequestDetailsRiRecipients>,
    /// Defines the total transaction amount.
    #[serde(rename = "totalTransactionAmount")]
    pub total_transaction_amount: String,
    /// Represents the unique identifier of a transaction, i.e. it could be transactionId in UTXO-based protocols like Bitcoin, and transaction hash in Ethereum blockchain.
    #[serde(rename = "transactionId", skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// Defines the status of the transaction request, e.g. pending.
    #[serde(rename = "transactionRequestStatus")]
    pub transaction_request_status: TransactionRequestStatus,
    /// Defines the transaction type, if it is for coins or tokens.
    #[serde(rename = "transactionType")]
    pub transaction_type: TransactionType,
    /// Defines the unit of the amount.
    #[serde(rename = "unit")]
    pub unit: String,
    /// Defines the unique ID of the Wallet.
    #[serde(rename = "walletId")]
    pub wallet_id: String,
}

impl GetTransactionRequestDetailsRi {
    pub fn new(additional_details: String, blockchain: Blockchain, fee_priority: FeePriority, network: Network, recipients: Vec<crate::models::GetTransactionRequestDetailsRiRecipients>, total_transaction_amount: String, transaction_request_status: TransactionRequestStatus, transaction_type: TransactionType, unit: String, wallet_id: String) -> GetTransactionRequestDetailsRi {
        GetTransactionRequestDetailsRi {
            additional_details,
            blockchain,
            fee_priority,
            network,
            recipients,
            total_transaction_amount,
            transaction_id: None,
            transaction_request_status,
            transaction_type,
            unit,
            wallet_id,
        }
    }
}

/// Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Blockchain {
    #[serde(rename = "bitcoin")]
    Bitcoin,
    #[serde(rename = "bitcoin-cash")]
    BitcoinCash,
    #[serde(rename = "litecoin")]
    Litecoin,
    #[serde(rename = "dogecoin")]
    Dogecoin,
    #[serde(rename = "dash")]
    Dash,
    #[serde(rename = "ethereum")]
    Ethereum,
    #[serde(rename = "ethereum-classic")]
    EthereumClassic,
    #[serde(rename = "xrp")]
    Xrp,
    #[serde(rename = "zcash")]
    Zcash,
}
/// Defines the priority for the fee, if it is \"slow\", \"standard\" or \"fast\".
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FeePriority {
    #[serde(rename = "slow")]
    Slow,
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "fast")]
    Fast,
}
/// Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Network {
    #[serde(rename = "mainnet")]
    Mainnet,
    #[serde(rename = "testnet")]
    Testnet,
    #[serde(rename = "ropsten")]
    Ropsten,
    #[serde(rename = "mordor")]
    Mordor,
}
/// Defines the status of the transaction request, e.g. pending.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransactionRequestStatus {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "await-approval")]
    AwaitApproval,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "prepared")]
    Prepared,
    #[serde(rename = "signed")]
    Signed,
    #[serde(rename = "broadcasted")]
    Broadcasted,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "mined")]
    Mined,
}
/// Defines the transaction type, if it is for coins or tokens.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransactionType {
    #[serde(rename = "coin")]
    Coin,
    #[serde(rename = "token")]
    Token,
}

