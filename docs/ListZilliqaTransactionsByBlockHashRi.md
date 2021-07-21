# ListZilliqaTransactionsByBlockHashRi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fee** | [**crate::models::GetZilliqaTransactionDetailsByTransactionIdriFee**](GetZilliqaTransactionDetailsByTransactionIDRI_fee.md) |  | 
**gas_limit** | **i32** | Represents the maximum amount of gas allowed in the block in order to determine how many transactions it can fit. | 
**gas_price** | **i32** | Defines the price of the gas. | 
**gas_used** | **i32** | Defines how much of the gas for the block has been used. | 
**mined_in_block_height** | **i32** | Represents the number of blocks in the blockchain preceding this specific block. Block numbers have no gaps. A blockchain usually starts with block 0 called the \"Genesis block\". | 
**nonce** | **i32** | Represents a random value that can be adjusted to satisfy the Proof of Work. | 
**recipients** | [**Vec<crate::models::ListZilliqaTransactionsByAddressRiRecipients>**](ListZilliqaTransactionsByAddressRI_recipients.md) | Defines an object array of the transaction recipients. | 
**senders** | [**Vec<crate::models::ListZilliqaTransactionsByAddressRiSenders>**](ListZilliqaTransactionsByAddressRI_senders.md) | Represents an object of addresses that provide the funds. | 
**timestamp** | **i32** | Defines the exact date/time when this block was mined in Unix Timestamp. | 
**transaction_hash** | **String** | Represents the hash of the transaction, which is its unique identifier. | 
**transaction_index** | **i32** | Defines the numeric representation of the transaction index. | 
**transaction_status** | **String** | Defines the status of the transaction, whether it is e.g. pending or complete. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


