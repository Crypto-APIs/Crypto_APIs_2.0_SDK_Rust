# GetWalletTransactionDetailsByTransactionIdribsbcVin

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**addresses** | **Vec<String>** |  | 
**coinbase** | Option<**String**> | Represents the coinbase hex. | [optional]
**script_sig** | [**crate::models::GetWalletTransactionDetailsByTransactionIdribsbcScriptSig**](GetWalletTransactionDetailsByTransactionIDRIBSBC_scriptSig.md) |  | 
**sequence** | **i64** | Represents the script sequence number. | 
**txid** | **String** | Represents the reference transaction identifier. | 
**txinwitness** | Option<**Vec<String>**> |  | [optional]
**value** | Option<**String**> | Represents the sent/received amount. | [optional]
**vout** | Option<**i32**> | It refers to the index of the output address of this transaction. The index starts from 0. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


