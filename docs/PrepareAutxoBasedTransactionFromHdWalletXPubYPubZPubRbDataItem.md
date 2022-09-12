# PrepareAutxoBasedTransactionFromHdWalletXPubYPubZPubRbDataItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**additional_data** | Option<**String**> | Representation of the additional data. | [optional]
**fee** | [**crate::models::PrepareAutxoBasedTransactionFromHdWalletXPubYPubZPubRbDataItemFee**](PrepareAUTXO_BasedTransactionFromHDWalletXPubYPubZPubRB_data_item_fee.md) |  | 
**locktime** | Option<**i32**> | Represents the time at which a particular transaction can be added to the blockchain. | [optional]
**prepare_strategy** | Option<**String**> | Representation of the transaction's strategy type | [optional]
**recipients** | [**Vec<crate::models::PrepareAutxoBasedTransactionFromHdWalletXPubYPubZPubRbDataItemRecipients>**](PrepareAUTXO_BasedTransactionFromHDWalletXPubYPubZPubRB_data_item_recipients.md) | Represents a list of recipient addresses with the respective amounts. In account-based protocols like Ethereum there is only one address in this list. | 
**replaceable** | Option<**bool**> | Representation of whether the transaction is replaceable | [optional]
**xpub** | **String** | Defines the account extended publicly known key which is used to derive all child public keys. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


