# ListXrpRippleTransactionsByAddressRi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**destination_tag** | Option<**i32**> |  | [optional]
**index** | **i32** | Represents the index position of the transaction in the block. | 
**mined_in_block_hash** | **String** | Represents the hash of the block where this transaction was mined/confirmed for first time. The hash is defined as a cryptographic digital fingerprint made by hashing the block header twice through the SHA256 algorithm. | 
**mined_in_block_height** | **i32** | Represents the hight of the block where this transaction was mined/confirmed for first time. The height is defined as the number of blocks in the blockchain preceding this specific block. | 
**recipients** | [**Vec<crate::models::GetXrpRippleTransactionDetailsByTransactionIdriRecipients>**](GetXRPRippleTransactionDetailsByTransactionIDRI_recipients.md) | Represents an object of addresses that receive the transactions. | 
**senders** | [**Vec<crate::models::GetXrpRippleTransactionDetailsByTransactionIdriSenders>**](GetXRPRippleTransactionDetailsByTransactionIDRI_senders.md) | Represents an object of addresses that provide the funds. | 
**sequence** | **i32** | Defines the transaction input's sequence as an integer, which is is used when transactions are replaced with newer versions before LockTime. | 
**status** | **String** | Defines the status of the transaction. | 
**timestamp** | **i32** | Defines the exact date/time in Unix Timestamp when this transaction was mined, confirmed or first seen in Mempool, if it is unconfirmed. | 
**transaction_hash** | **String** | Represents the hash of the XRP transaction. | 
**_type** | **String** | Specifies the type of the transaction. | 
**fee** | [**crate::models::ListXrpRippleTransactionsByAddressRiFee**](ListXRPRippleTransactionsByAddressRI_fee.md) |  | 
**offer** | [**crate::models::ListXrpRippleTransactionsByAddressRiOffer**](ListXRPRippleTransactionsByAddressRI_offer.md) |  | 
**receive** | [**crate::models::ListXrpRippleTransactionsByAddressRiReceive**](ListXRPRippleTransactionsByAddressRI_receive.md) |  | 
**value** | [**crate::models::ListXrpRippleTransactionsByAddressRiValue**](ListXRPRippleTransactionsByAddressRI_value.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


