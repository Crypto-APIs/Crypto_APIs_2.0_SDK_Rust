# PrepareAnAccountBasedTransactionFromHdWalletXPubYPubZPubRi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | **String** | Representation of the amount of the transaction | 
**data_hex** | **String** | Representation of the data in hex value | 
**derivation_index** | **i64** | Representation of the derivation index of the xpub address | 
**fee** | [**crate::models::PrepareAnAccountBasedTransactionFromHdWalletXPubYPubZPubRiFee**](PrepareAnAccount_BasedTransactionFromHDWalletXPubYPubZPubRI_fee.md) |  | 
**nonce** | **String** | Represents the sequential running number for an address, starting from 0 for the first transaction. E.g., if the nonce of a transaction is 10, it would be the 11th transaction sent from the sender's address. | 
**recipient** | **String** | Represents a recipient addresses. In account-based protocols like Ethereum there is only one address in this list. | 
**sender** | **String** | Represents a sender address. In account-based protocols like Ethereum there is only one address in this list. | 
**sig_hash** | **String** | Representation of the hash that should be signed. | 
**transaction_type** | **String** | Representation of the transaction type | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


