# GetTransactionDetailsByTransactionIdFromCallbackRi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**index** | **i32** | Represents the index position of the transaction in the specific block. | 
**mined_in_block_hash** | **String** | Represents the hash of the block where this transaction was mined/confirmed for first time. The hash is defined as a cryptographic digital fingerprint made by hashing the block header twice through the SHA256 algorithm. | 
**mined_in_block_height** | **i32** | Represents the hight of the block where this transaction was mined/confirmed for first time. The height is defined as the number of blocks in the blockchain preceding this specific block. | 
**recipients** | [**Vec<crate::models::GetTransactionDetailsByTransactionIdFromCallbackRiRecipients>**](GetTransactionDetailsByTransactionIDFromCallbackRI_recipients.md) | Represents a list of recipient addresses with the respective amounts. In account-based protocols like Ethereum there is only one address in this list. | 
**senders** | [**Vec<crate::models::GetTransactionDetailsByTransactionIdFromCallbackRiSenders>**](GetTransactionDetailsByTransactionIDFromCallbackRI_senders.md) | Represents a list of sender addresses with the respective amounts. In account-based protocols like Ethereum there is only one address in this list. | 
**timestamp** | **i32** | Defines the exact date/time in Unix Timestamp when this transaction was mined, confirmed or first seen in Mempool, if it is unconfirmed. | 
**transaction_hash** | **String** | Represents the same as transactionId for account-based protocols like Ethereum, while it could be different in UTXO-based protocols like Bitcoin. E.g., in UTXO-based protocols hash is different from transactionId for SegWit transactions. | 
**fee** | [**crate::models::GetTransactionDetailsByTransactionIdFromCallbackRiFee**](GetTransactionDetailsByTransactionIDFromCallbackRI_fee.md) |  | 
**is_confirmed** | **bool** | Represents the state of the transaction whether it is confirmed or not confirmed. | 
**blockchain_specific** | [**crate::models::GetTransactionDetailsByTransactionIdFromCallbackRibs**](GetTransactionDetailsByTransactionIDFromCallbackRIBS.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


