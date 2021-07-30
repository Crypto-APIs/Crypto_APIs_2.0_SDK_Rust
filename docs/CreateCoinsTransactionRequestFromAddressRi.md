# CreateCoinsTransactionRequestFromAddressRi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**callback_secret_key** | Option<**String**> | Represents the Secret Key value provided by the customer. This field is used for security purposes during the callback notification, in order to prove the sender of the callback as Crypto APIs. | [optional]
**callback_url** | Option<**String**> | Verified URL for sending callbacks | [optional]
**fee_priority** | **String** | Represents the fee priority of the automation, whether it is \"slow\", \"standard\" or \"fast\". | 
**recipients** | [**Vec<crate::models::CreateCoinsTransactionRequestFromAddressRiRecipients>**](CreateCoinsTransactionRequestFromAddressRI_recipients.md) | Defines the destination for the transaction, i.e. the recipient(s). | 
**senders** | [**crate::models::CreateCoinsTransactionRequestFromAddressRiSenders**](CreateCoinsTransactionRequestFromAddressRI_senders.md) |  | 
**transaction_request_status** | **String** | Defines the status of the transaction request, e.g. \"created, \"await_approval\", \"pending\", \"prepared\", \"signed\", \"broadcasted\", \"success\", \"failed\", \"rejected\", mined\". | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


