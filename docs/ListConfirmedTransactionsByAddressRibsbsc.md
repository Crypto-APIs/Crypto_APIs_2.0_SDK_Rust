# ListConfirmedTransactionsByAddressRibsbsc

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**contract** | Option<**String**> | Represents the specific transaction contract. | [optional]
**gas_limit** | **String** | Represents the amount of gas used by this specific transaction alone. | 
**gas_price** | [**crate::models::ListConfirmedTransactionsByAddressRibsbscGasPrice**](ListConfirmedTransactionsByAddressRIBSBSC_gasPrice.md) |  | 
**gas_used** | **String** | Represents the exact unit of gas that was used for the transaction. | 
**input_data** | **String** | Represents additional information that is required for the transaction. | 
**internal_transactions_count** | **i32** | Represents the total internal transactions count. | 
**nonce** | **i32** | Represents the sequential running number for an address, starting from 0 for the first transaction. E.g., if the nonce of a transaction is 10, it would be the 11th transaction sent from the sender's address. | 
**token_transfers_count** | **i32** | Represents the total token transfers count. | 
**transaction_status** | **String** | String representation of the transaction status | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


