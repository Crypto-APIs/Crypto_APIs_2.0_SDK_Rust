# CreateCoinsTransactionRequestFromAddressRi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address_tag** | Option<**i32**> | Defines a specific Tag that is an additional XRP address feature. It helps identify a transaction recipient beyond a wallet address. The tag that was encoded into the x-Address along with the Source Classic Address. | [optional]
**callback_secret_key** | Option<**String**> | Represents the Secret Key value provided by the customer. This field is used for security purposes during the callback notification, in order to prove the sender of the callback as Crypto APIs. For more information please see our [Documentation](https://developers.cryptoapis.io/technical-documentation/general-information/callbacks#callback-security). | [optional]
**callback_url** | Option<**String**> | Represents the URL that is set by the customer where the callback will be received at. The callback notification will be received only if and when the event occurs. `We support ONLY httpS type of protocol`. | [optional]
**classic_address** | Option<**String**> | Represents the public address, which is a compressed and shortened form of a public key. The classic address is shown when the source address is an x-Address. | [optional]
**fee_priority** | **String** | Represents the fee priority of the automation, whether it is \"slow\", \"standard\" or \"fast\". | 
**note** | Option<**String**> | Represents an optional note to add a free text in, explaining or providing additional detail on the transaction request. | [optional]
**recipients** | [**Vec<crate::models::CreateCoinsTransactionRequestFromAddressRiRecipients>**](CreateCoinsTransactionRequestFromAddressRI_recipients.md) | Defines the destination for the transaction, i.e. the recipient(s). | 
**senders** | [**crate::models::CreateCoinsTransactionRequestFromAddressRiSenders**](CreateCoinsTransactionRequestFromAddressRI_senders.md) |  | 
**transaction_request_id** | **String** | Represents a unique identifier of the transaction request (the request sent to make a transaction), which helps in identifying which callback and which `referenceId` concern that specific transaction request. | 
**transaction_request_status** | **String** | Defines the status of the transaction request, e.g. \"created, \"await_approval\", \"pending\", \"prepared\", \"signed\", \"broadcasted\", \"success\", \"failed\", \"rejected\", mined\". | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


