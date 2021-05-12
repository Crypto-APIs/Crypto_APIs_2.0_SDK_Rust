# ListXrpRippleTransactionsByBlockHashResponseItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**additional_data** | Option<**String**> | Represents any additional data that may be needed. | [optional]
**index** | **i32** | Represents the index position of the transaction in the specific block. | 
**mined_in_block_hash** | **String** | Represents the hash of the block where this transaction was mined/confirmed for first time. The hash is defined as a cryptographic digital fingerprint made by hashing the block header twice through the SHA256 algorithm. | 
**mined_in_block_height** | **i32** | Represents the hight of the block where this transaction was mined/confirmed for first time. The height is defined as the number of blocks in the blockchain preceding this specific block. | 
**recipients** | [**Vec<crate::models::ListXrpRippleTransactionsByBlockHashResponseItemRecipients>**](ListXRPRippleTransactionsByBlockHashResponseItem_recipients.md) | Represents an object of addresses that receive the transactions. | 
**senders** | [**Vec<crate::models::ListXrpRippleTransactionsByBlockHashResponseItemSenders>**](ListXRPRippleTransactionsByBlockHashResponseItem_senders.md) | Represents an object of addresses that provide the funds. | 
**sequence** | **i32** | Defines the transaction input's sequence as an integer, which is is used when transactions are replaced with newer versions before LockTime. | 
**status** | **String** | Defines the status of the transaction. | 
**timestamp** | **i32** | Defines the exact date/time in Unix Timestamp when this transaction was mined, confirmed or first seen in Mempool, if it is unconfirmed. | 
**transaction_hash** | **String** | Represents the same as `transactionId` for account-based protocols like Ethereum, while it could be different in UTXO-based protocols like Bitcoin. E.g., in UTXO-based protocols `hash` is different from `transactionId` for SegWit transactions. | 
**_type** | **String** | Defines the type of the transaction. | 
**fee** | [**crate::models::ListXrpRippleTransactionsByBlockHashResponseItemFee**](ListXRPRippleTransactionsByBlockHashResponseItem_fee.md) |  | 
**offer** | [**crate::models::ListXrpRippleTransactionsByBlockHashResponseItemOffer**](ListXRPRippleTransactionsByBlockHashResponseItem_offer.md) |  | 
**receive** | [**crate::models::ListXrpRippleTransactionsByBlockHashResponseItemReceive**](ListXRPRippleTransactionsByBlockHashResponseItem_receive.md) |  | 
**value** | [**crate::models::ListXrpRippleTransactionsByBlockHashResponseItemValue**](ListXRPRippleTransactionsByBlockHashResponseItem_value.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


