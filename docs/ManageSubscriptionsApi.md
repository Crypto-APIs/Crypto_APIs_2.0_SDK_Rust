# \ManageSubscriptionsApi

All URIs are relative to *https://rest.cryptoapis.io/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_blockchain_event_subscription**](ManageSubscriptionsApi.md#delete_blockchain_event_subscription) | **delete** /blockchain-events/{blockchain}/{network}/subscriptions/{referenceId} | Delete Blockchain Event Subscription
[**list_blockchain_events_subscriptions**](ManageSubscriptionsApi.md#list_blockchain_events_subscriptions) | **get** /blockchain-events/{blockchain}/{network}/subscriptions | List Blockchain Events Subscriptions



## delete_blockchain_event_subscription

> crate::models::DeleteBlockchainEventSubscriptionResponse delete_blockchain_event_subscription(blockchain, network, reference_id, context)
Delete Blockchain Event Subscription

Through this endpoint the customer can delete blockchain event subscriptions they have by attributes `referenceId` and `network`.    Currently Crypto APIs 2.0 offers certain Blockchain event endpoints which allow the user to subscribe for one/a few/all and receive callback notifications when the specific event occurs.    {note}To have an operational callback subscription, you need to first verify a domain for the Callback URL. Please see more information on Callbacks [here](https://developers.cryptoapis.io/technical-documentation/general-information/callbacks#callback-url).{/note}    {warning}Crypto APIs will notify the user **only when** the event occurs. There are cases when the specific event doesn't happen at all, or takes a long time to do so. A callback notification **will not** be sent if the event does not or cannot occur, or will take long time to occur.{/warning}

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\", \"rinkeby\" are test networks. | [required] |
**reference_id** | **String** | Represents a unique ID used to reference the specific callback subscription. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::DeleteBlockchainEventSubscriptionResponse**](DeleteBlockchainEventSubscriptionResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_blockchain_events_subscriptions

> crate::models::ListBlockchainEventsSubscriptionsResponse list_blockchain_events_subscriptions(blockchain, network, context, limit, offset)
List Blockchain Events Subscriptions

Through this endpoint the customer can obtain a list of their callback subscriptions for the available Blockchain events.    Currently Crypto APIs 2.0 offers certain Blockchain event endpoints which allow the user to subscribe for one/a few/all and receive callback notifications when the specific event occurs.    {note}To have an operational callback subscription, you need to first verify a domain for the Callback URL. Please see more information on Callbacks [here](https://developers.cryptoapis.io/technical-documentation/general-information/callbacks#callback-url).{/note}    {warning}Crypto APIs will notify the user **only when** the event occurs. There are cases when the specific event doesn't happen at all, or takes a long time to do so. A callback notification **will not** be sent if the event does not or cannot occur, or will take long time to occur.{/warning}

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\", \"rinkeby\" are test networks. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**limit** | Option<**i32**> | Defines how many items should be returned in the response per page basis. |  |[default to 50]
**offset** | Option<**i32**> | The starting index of the response items, i.e. where the response should start listing the returned items. |  |[default to 0]

### Return type

[**crate::models::ListBlockchainEventsSubscriptionsResponse**](ListBlockchainEventsSubscriptionsResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
