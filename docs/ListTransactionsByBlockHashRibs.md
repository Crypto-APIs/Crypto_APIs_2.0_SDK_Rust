# ListTransactionsByBlockHashRibs

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**locktime** | **i32** | Represents the time at which a particular transaction can be added to the blockchain. | 
**size** | **i32** | Represents the total size of this transaction. | 
**v_size** | **i32** | Represents the virtual size of this transaction. | 
**version** | **i32** | Represents transaction version number. | 
**vin** | [**Vec<crate::models::ListTransactionsByBlockHashRibsd2Vin>**](ListTransactionsByBlockHashRIBSD2_vin.md) | Represents the transaction inputs. | 
**vout** | [**Vec<crate::models::ListTransactionsByBlockHashRibsd2Vout>**](ListTransactionsByBlockHashRIBSD2_vout.md) | Represents the transaction outputs. | 
**vsize** | **i32** | Represents the virtual size of this transaction. | 
**contract** | **String** | Represents the specific transaction contract. | 
**gas_limit** | **String** | Represents the amount of gas used by this specific transaction alone. | 
**gas_price** | [**crate::models::ListTransactionsByBlockHashRibseGasPrice**](ListTransactionsByBlockHashRIBSE_gasPrice.md) |  | 
**gas_used** | **String** | Represents the exact unit of gas that was used for the transaction. | 
**input_data** | **String** | Represents additional information that is required for the transaction. | 
**nonce** | **String** | Represents the sequential running number for an address, starting from 0 for the first transaction. E.g., if the nonce of a transaction is 10, it would be the 11th transaction sent from the sender's address. | 
**transaction_status** | **String** | String representation of the transaction status | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


