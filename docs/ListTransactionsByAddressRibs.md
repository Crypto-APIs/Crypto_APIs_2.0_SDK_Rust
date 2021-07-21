# ListTransactionsByAddressRibs

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**locktime** | **i32** | Represents the locktime on the transaction on the specific blockchain, i.e. the blockheight at which the transaction is valid. | 
**size** | **i32** | Represents the total size of this transaction. | 
**v_size** | **i32** | Represents the virtual size of this transaction. | 
**version** | **i32** | Represents the transaction's version number. | 
**vin** | [**Vec<crate::models::ListTransactionsByAddressRibsd2Vin>**](ListTransactionsByAddressRIBSD2_vin.md) | Represents the transaction inputs. | 
**vout** | [**Vec<crate::models::ListTransactionsByAddressRibsd2Vout>**](ListTransactionsByAddressRIBSD2_vout.md) | Represents the transaction outputs. | 
**contract** | **String** | Represents the specific transaction contract. | 
**gas_limit** | **String** | Represents the amount of gas used by this specific transaction alone. | 
**gas_price** | [**crate::models::ListTransactionsByAddressRibseGasPrice**](ListTransactionsByAddressRIBSE_gasPrice.md) |  | 
**gas_used** | **String** | Represents the exact unit of gas that was used for the transaction. | 
**input_data** | **String** | Represents additional information that is required for the transaction. | 
**nonce** | **i32** | Represents the sequential running number for an address, starting from 0 for the first transaction. E.g., if the nonce of a transaction is 10, it would be the 11th transaction sent from the sender's address. | 
**transaction_status** | **String** | String representation of the transaction status | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


