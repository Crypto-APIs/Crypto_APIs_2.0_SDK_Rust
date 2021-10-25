# GetLastMinedBlockRi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**hash** | **String** | Represents the hash of the block, which is its unique identifier. It represents a cryptographic digital fingerprint made by hashing the block header twice through the SHA256 algorithm. | 
**height** | **i32** | Represents the number of blocks in the blockchain preceding this specific block. Block numbers have no gaps. A blockchain usually starts with block 0 called the \"Genesis block\". | 
**previous_block_hash** | **String** | Represents the hash of the previous block, also known as the parent block. | 
**timestamp** | **i32** | Defines the exact date/time when this block was mined in Unix Timestamp. | 
**transactions_count** | **i32** | Represents the total number of all transactions as part of this block. | 
**blockchain_specific** | [**crate::models::GetLastMinedBlockRibs**](GetLastMinedBlockRIBS.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


