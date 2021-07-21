# GetXrpRippleBlockDetailsByBlockHeightRi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**block_hash** | **String** | Represents the hash of the block, which is its unique identifier. It represents a cryptographic digital fingerprint made by hashing the block header twice through the SHA256 algorithm. | 
**block_height** | **i32** | Represents the number of blocks in the blockchain preceding this specific block. Block numbers have no gaps. A blockchain usually starts with block 0 called the \"Genesis block\". | 
**next_block_hash** | **String** | Represents the hash of the next block. When this is the last block of the blockchain this value will be an empty string. | 
**previous_block_hash** | **String** | Represents the hash of the previous block, also known as the parent block. | 
**timestamp** | **i32** | Defines the exact date/time when this block was mined in Unix Timestamp. | 
**total_coins** | [**crate::models::GetXrpRippleBlockDetailsByBlockHeightRiTotalCoins**](GetXRPRippleBlockDetailsByBlockHeightRI_totalCoins.md) |  | 
**total_fees** | [**crate::models::GetXrpRippleBlockDetailsByBlockHeightRiTotalFees**](GetXRPRippleBlockDetailsByBlockHeightRI_totalFees.md) |  | 
**transactions_count** | **i32** | Represents the total number of all transactions as part of this block. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


