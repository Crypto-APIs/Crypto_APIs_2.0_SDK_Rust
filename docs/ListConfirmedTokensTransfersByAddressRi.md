# ListConfirmedTokensTransfersByAddressRi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**contract_address** | **String** | Represents the contract address of the token, which controls its logic. It is not the address that holds the tokens. | 
**mined_in_block_height** | **i64** | Defines the block height in which this transaction was confirmed/mined. | 
**recipient_address** | **String** | Defines the address to which the recipient receives the transferred tokens. | 
**sender_address** | **String** | Defines the address from which the sender transfers tokens. | 
**token_decimals** | **i32** | Defines the decimals of the token, i.e. the number of digits that come after the decimal coma of the token. | 
**token_id** | Option<**String**> | Represents the unique token identifier. | [optional]
**token_name** | **String** | Defines the token's name as a string. | 
**token_symbol** | **String** | Defines the token symbol by which the token contract is known. It is usually 3-4 characters in length. | 
**token_type** | **String** | Defines the specific token type. | 
**tokens_amount** | Option<**String**> | Defines the token amount of the transfer. | [optional]
**transaction_hash** | **String** | Represents the hash of the transaction, which is its unique identifier. It represents a cryptographic digital fingerprint made by hashing the block header twice through the SHA256 algorithm. | 
**transaction_timestamp** | **i32** | Defines the specific time/date when the transaction was created in Unix Timestamp. | 
**transaction_fee** | [**crate::models::ListTokensTransfersByTransactionHashRiTransactionFee**](ListTokensTransfersByTransactionHashRI_transactionFee.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


