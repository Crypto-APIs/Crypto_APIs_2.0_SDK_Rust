# ListWalletTransactionsRi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**direction** | **String** | Defines the direction of the transaction, e.g. incoming. | 
**fee** | [**crate::models::ListWalletTransactionsRiFee**](ListWalletTransactionsRI_fee.md) |  | 
**fungible_tokens** | Option<[**Vec<crate::models::ListWalletTransactionsRiFungibleTokens>**](ListWalletTransactionsRI_fungibleTokens.md)> | Represents fungible tokens'es detailed information | [optional]
**internal_transactions** | Option<[**Vec<crate::models::ListWalletTransactionsRiInternalTransactions>**](ListWalletTransactionsRI_internalTransactions.md)> |  | [optional]
**non_fungible_tokens** | Option<[**Vec<crate::models::ListWalletTransactionsRiNonFungibleTokens>**](ListWalletTransactionsRI_nonFungibleTokens.md)> | Represents non-fungible tokens'es detailed information. | [optional]
**recipients** | [**Vec<crate::models::ListWalletTransactionsRiRecipients>**](ListWalletTransactionsRI_recipients.md) | Represents a list of recipient addresses with the respective amounts. In account-based protocols like Ethereum there is only one address in this list. | 
**senders** | [**Vec<crate::models::ListWalletTransactionsRiSenders>**](ListWalletTransactionsRI_senders.md) | Represents a list of sender addresses with the respective amounts. In account-based protocols like Ethereum there is only one address in this list. | 
**status** | **String** | Defines the status of the transaction, if it is confirmed or unconfirmed. | 
**timestamp** | **i32** | Defines the exact date/time in Unix Timestamp when this transaction was mined, confirmed or first seen in Mempool, if it is unconfirmed. | 
**transaction_id** | **String** | Represents the unique TD of the transaction. | 
**value** | [**crate::models::ListWalletTransactionsRiValue**](ListWalletTransactionsRI_value.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


