# GetTransactionRequestDetailsRi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**additional_details** | **String** | Defines an optional note for additional details. | 
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | 
**fee_priority** | **String** | Defines the priority for the fee, if it is \"slow\", \"standard\" or \"fast\". | 
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | 
**recipients** | [**Vec<crate::models::GetTransactionRequestDetailsRiRecipients>**](GetTransactionRequestDetailsRI_recipients.md) | Represents a list of recipient addresses with the respective amounts. In account-based protocols like Ethereum there is only one address in this list. | 
**total_transaction_amount** | **String** | Defines the total transaction amount. | 
**transaction_id** | Option<**String**> | Represents the unique identifier of a transaction, i.e. it could be transactionId in UTXO-based protocols like Bitcoin, and transaction hash in Ethereum blockchain. | [optional]
**transaction_request_status** | **String** | Defines the status of the transaction request, e.g. pending. | 
**transaction_type** | **String** | Defines the transaction type, if it is for coins or tokens. | 
**unit** | **String** | Defines the unit of the amount. | 
**wallet_id** | **String** | Defines the unique ID of the Wallet. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


