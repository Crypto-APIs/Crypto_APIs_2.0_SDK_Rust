# ListXrpRippleTransactionsByBlockHashRi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**additional_data** | Option<**String**> | Represents any additional data that may be needed. | [optional]
**destination_tag** | Option<**i64**> |  | [optional]
**index** | **i32** | Represents the index position of the transaction in the specific block. | 
**mined_in_block_height** | **i32** | Represents the hight of the block where this transaction was mined/confirmed for first time. The height is defined as the number of blocks in the blockchain preceding this specific block. | 
**recipients** | [**Vec<crate::models::ListXrpRippleTransactionsByBlockHashRiRecipients>**](ListXRPRippleTransactionsByBlockHashRI_recipients.md) | Represents an object of addresses that receive the transactions. | 
**senders** | [**Vec<crate::models::ListXrpRippleTransactionsByBlockHashRiSenders>**](ListXRPRippleTransactionsByBlockHashRI_senders.md) | Represents an object of addresses that provide the funds. | 
**sequence** | **i64** | Defines the transaction input's sequence as an integer, which is is used when transactions are replaced with newer versions before LockTime. | 
**status** | **String** | Defines the status of the transaction. | 
**timestamp** | **i32** | Defines the exact date/time in Unix Timestamp when this transaction was mined, confirmed or first seen in Mempool, if it is unconfirmed. | 
**transaction_hash** | **String** | Represents the same as `transactionId` for account-based protocols like Ethereum, while it could be different in UTXO-based protocols like Bitcoin. E.g., in UTXO-based protocols `hash` is different from `transactionId` for SegWit transactions. | 
**_type** | **String** | Defines the type of the transaction. | 
**fee** | [**crate::models::ListXrpRippleTransactionsByBlockHashRiFee**](ListXRPRippleTransactionsByBlockHashRI_fee.md) |  | 
**offer** | [**crate::models::ListXrpRippleTransactionsByBlockHashRiOffer**](ListXRPRippleTransactionsByBlockHashRI_offer.md) |  | 
**receive** | [**crate::models::ListXrpRippleTransactionsByBlockHashRiReceive**](ListXRPRippleTransactionsByBlockHashRI_receive.md) |  | 
**value** | [**crate::models::ListXrpRippleTransactionsByBlockHashRiValue**](ListXRPRippleTransactionsByBlockHashRI_value.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


