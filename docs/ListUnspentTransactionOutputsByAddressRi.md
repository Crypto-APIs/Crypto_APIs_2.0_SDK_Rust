# ListUnspentTransactionOutputsByAddressRi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**index** | **i32** | Represents the index position of the transaction in the block. | 
**locktime** | **i32** | Represents the time at which a particular transaction can be added to the blockchain | 
**mined_in_block_hash** | **String** | Represents the hash of the block where this transaction was mined/confirmed for first time. The hash is defined as a cryptographic digital fingerprint made by hashing the block header twice through the SHA256 algorithm. | 
**mined_in_block_height** | **i32** | Represents the hight of the block where this transaction was mined/confirmed for first time. The height is defined as the number of blocks in the blockchain preceding this specific block. | 
**recipients** | [**Vec<crate::models::GetTransactionDetailsByTransactionIdriRecipients>**](GetTransactionDetailsByTransactionIDRI_recipients.md) | Represents a list of recipient addresses with the respective amounts. In account-based protocols like Ethereum there is only one address in this list. | 
**senders** | [**Vec<crate::models::ListUnspentTransactionOutputsByAddressRiSenders>**](ListUnspentTransactionOutputsByAddressRI_senders.md) | Object Array representation of transaction senders | 
**size** | **i32** | Represents the total size of this transaction | 
**timestamp** | **i32** | Defines the exact date/time in Unix Timestamp when this transaction was mined, confirmed or first seen in Mempool, if it is unconfirmed. | 
**transaction_hash** | **String** | Represents the same as `transactionId` for account-based protocols like Ethereum, while it could be different in UTXO-based protocols like Bitcoin. E.g., in UTXO-based protocols `hash` is different from `transactionId` for SegWit transactions. | 
**transaction_id** | **String** | Represents the unique identifier of a transaction, i.e. it could be `transactionId` in UTXO-based protocols like Bitcoin, and transaction `hash` in Ethereum blockchain. | 
**version** | **i32** | Represents the transaction version number. | 
**vin** | [**Vec<crate::models::ListUnspentTransactionOutputsByAddressRiVin>**](ListUnspentTransactionOutputsByAddressRI_vin.md) | Represents the transaction inputs. | 
**vout** | [**Vec<crate::models::ListConfirmedTransactionsByAddressRibsbVout>**](ListConfirmedTransactionsByAddressRIBSB_vout.md) | Represents the transaction outputs. | 
**fee** | [**crate::models::ListUnspentTransactionOutputsByAddressRiFee**](ListUnspentTransactionOutputsByAddressRI_fee.md) |  | 
**blockchain_specific** | [**crate::models::ListUnspentTransactionOutputsByAddressRiBlockchainSpecific**](ListUnspentTransactionOutputsByAddressRI_blockchainSpecific.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


