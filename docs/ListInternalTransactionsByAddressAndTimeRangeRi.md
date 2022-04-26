# ListInternalTransactionsByAddressAndTimeRangeRi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | **String** | Defines the specific amount of the transaction. | 
**mined_in_block_hash** | **String** | Represents the hash of the block where this transaction was mined/confirmed for first time. The hash is defined as a cryptographic digital fingerprint made by hashing the block header twice through the SHA256 algorithm. | 
**mined_in_block_height** | **i32** | Represents the hight of the block where this transaction was mined/confirmed for first time. The height is defined as the number of blocks in the blockchain preceding this specific block. | 
**operation_id** | **String** | Represents the unique internal transaction ID in regards to the parent transaction (type trace address). | 
**operation_type** | **String** | Defines the call type of the internal transaction. | 
**parent_hash** | **String** | Defines the specific hash of the parent transaction. | 
**recipient** | **String** | Represents the recipient address with the respective amount. | 
**sender** | **String** | Represents the sender address with the respective amount. | 
**timestamp** | **i32** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


