# TransactionRequestBroadcastedDataItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | 
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\", \"rinkeby\" are test networks. | 
**required_approvals** | **i32** | The required number of approvals needed to approve the transaction. | 
**required_rejections** | **i32** | The required number of rejections needed to reject the transaction. | 
**current_approvals** | **i32** | The current number of approvals given for the transaction. | 
**current_rejections** | **i32** | The current number of rejections given for the transaction. | 
**transaction_id** | **String** | Defines the unique ID of the specific transaction, i.e. its identification number. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


