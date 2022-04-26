# DecodeRawTransactionHexRisd2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**locktime** | **i32** | Represents the time at which a particular transaction can be added to the blockchain. | 
**transaction_hash** | **String** | Represents the same as transactionId for account-based protocols like Ethereum, while it could be different in UTXO-based protocols like Bitcoin. E.g., in UTXO-based protocols hash is different from transactionId for SegWit transactions. | 
**v_size** | **i32** | Represents the virtual size of this transaction. | 
**version** | **i32** | Represents transaction version number | 
**vin** | [**Vec<crate::models::DecodeRawTransactionHexRisd2Vin>**](DecodeRawTransactionHexRISD2_vin.md) | Represents the transaction inputs. | 
**vout** | [**Vec<crate::models::DecodeRawTransactionHexRisd2Vout>**](DecodeRawTransactionHexRISD2_vout.md) | Represents the transaction outputs. | 
**weight** | Option<**i32**> | Represents the size of a block, measured in weight units and including the segwit discount. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


