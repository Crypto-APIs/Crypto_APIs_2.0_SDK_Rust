# CreateTokensTransactionRequestFromAddressRi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**callback_secret_key** | **String** | Represents the Secret Key value provided by the customer. This field is used for security purposes during the callback notification, in order to prove the sender of the callback as Crypto APIs. | 
**callback_url** | **String** | Verified URL for sending callbacks | 
**fee_priority** | **String** | Represents the fee priority of the automation, whether it is \"slow\", \"standard\" or \"fast\". | 
**recipients** | [**Vec<crate::models::CreateTokensTransactionRequestFromAddressRiRecipients>**](CreateTokensTransactionRequestFromAddressRI_recipients.md) | Defines the destination for the transaction, i.e. the recipient(s). | 
**senders** | [**crate::models::CreateTokensTransactionRequestFromAddressRiSenders**](CreateTokensTransactionRequestFromAddressRI_senders.md) |  | 
**token_type_specific_data** | [**crate::models::CreateTokensTransactionRequestFromAddressRis**](CreateTokensTransactionRequestFromAddressRIS.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


