# PrepareAutxoBasedTransactionFromHdWalletXPubYPubZPubRi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**additional_data** | Option<**String**> | Representation of the additional data | [optional]
**fee** | **String** | When isConfirmed is True - Defines the amount of the transaction fee When isConfirmed is False - For ETH-based blockchains this attribute represents the max fee value. | 
**fee_per_byte** | Option<**String**> | Defines the fee per byte value | [optional]
**locktime** | **i64** | Represents the time at which a particular transaction can be added to the blockchain. | 
**replaceable** | **bool** | Representation of whether the transaction is replaceable | 
**size** | **i32** | Represents the total size of this transaction. | 
**vin** | [**Vec<crate::models::PrepareAutxoBasedTransactionFromHdWalletXPubYPubZPubRiVin>**](PrepareAUTXO_BasedTransactionFromHDWalletXPubYPubZPubRI_vin.md) | Represents the transaction inputs. | 
**vout** | [**Vec<crate::models::PrepareAutxoBasedTransactionFromHdWalletXPubYPubZPubRiVout>**](PrepareAUTXO_BasedTransactionFromHDWalletXPubYPubZPubRI_vout.md) | Represents the transaction outputs. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


