# CreateTokensTransactionRequestFromAddressRbDataItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | **String** | Represents the specific amount of the transaction. | 
**callback_secret_key** | Option<**String**> | Represents the Secret Key value provided by the customer. This field is used for security purposes during the callback notification, in order to prove the sender of the callback as Crypto APIs. For more information please see our [Documentation](https://developers.cryptoapis.io/technical-documentation/general-information/callbacks#callback-security). | [optional]
**callback_url** | Option<**String**> | Represents the URL that is set by the customer where the callback will be received at. The callback notification will be received only if and when the event occurs. | [optional]
**fee_priority** | **String** | Represents the fee priority of the automation, whether it is \"slow\", \"standard\" or \"fast\". | 
**note** | Option<**String**> | Represents an optional note to add a free text in, explaining or providing additional detail on the transaction request. | [optional]
**recipient_address** | **String** | Defines the specific recipient address for the transaction. | 
**token_identifier** | **String** | Defines the specific token identifier. For Bitcoin-based transactions it should be the `propertyId` and for Ethereum-based transactions - the `contract`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


