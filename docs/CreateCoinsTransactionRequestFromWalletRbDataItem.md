# CreateCoinsTransactionRequestFromWalletRbDataItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**callback_secret_key** | Option<**String**> | Represents the Secret Key value provided by the customer. This field is used for security purposes during the callback notification, in order to prove the sender of the callback as Crypto APIs. For more information please see our [Documentation](https://developers.cryptoapis.io/technical-documentation/general-information/callbacks#callback-security). | [optional]
**callback_url** | Option<**String**> | Represents the URL that is set by the customer where the callback will be received at. The callback notification will be received only if and when the event occurs. `We support ONLY httpS type of protocol`. | [optional]
**fee_priority** | **String** | Represents the fee priority of the automation, whether it is \"slow\", \"standard\" or \"fast\". | 
**note** | Option<**String**> | Represents an optional note to add a free text in, explaining or providing additional detail on the transaction request. | [optional]
**prepare_strategy** | Option<**String**> | Refers to a model of a UTXO spending strategy, where customers can choose how to spend their transaction outputs from multiple Bitcoin addresses. Two options available - \"minimize-dust\" (select lower amounts from multiple addresses) or \"optimize-size\" (select higher amounts from less addresses). | [optional][default to PrepareStrategy_MinimizeDust]
**recipients** | [**Vec<crate::models::CreateCoinsTransactionRequestFromWalletRbDataItemRecipients>**](CreateCoinsTransactionRequestFromWalletRB_data_item_recipients.md) | Defines the destination of the transaction, whether it is incoming or outgoing. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


