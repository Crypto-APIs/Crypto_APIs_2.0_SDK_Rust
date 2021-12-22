# ListAllUnconfirmedTransactionsRi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**recipients** | [**Vec<crate::models::ListUnconfirmedTransactionsByAddressRiRecipients>**](ListUnconfirmedTransactionsByAddressRI_recipients.md) | Represents a list of recipient addresses with the respective amounts. In account-based protocols like Ethereum there is only one address in this list. | 
**senders** | [**Vec<crate::models::ListUnconfirmedTransactionsByAddressRiSenders>**](ListUnconfirmedTransactionsByAddressRI_senders.md) | Represents a list of sender addresses with the respective amounts. In account-based protocols like Ethereum there is only one address in this list. | 
**timestamp** | **i32** | Defines the exact date/time in Unix Timestamp when this transaction was mined, confirmed or first seen in Mempool, if it is unconfirmed. | 
**transaction_hash** | **String** | String representation of the transaction hash | 
**transaction_id** | **String** | Represents the unique identifier of a transaction, i.e. it could be `transactionId` in UTXO-based protocols like Bitcoin, and transaction `hash` in Ethereum blockchain. | 
**blockchain_specific** | [**crate::models::ListAllUnconfirmedTransactionsRibs**](ListAllUnconfirmedTransactionsRIBS.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


