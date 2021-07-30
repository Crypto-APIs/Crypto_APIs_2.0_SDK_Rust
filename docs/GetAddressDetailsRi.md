# GetAddressDetailsRi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transactions_count** | **i32** | Represents the total number of confirmed coins transactions for this address, both incoming and outgoing. Applies for coins only **and not** tokens transfers e.g. for Ethereum. `transactionsCount` could result as less than incoming and outgoing transactions put together (e.g. in Bitcoin), due to the fact that one and the same address could be in senders and receivers addresses. | 
**confirmed_balance** | [**crate::models::GetAddressDetailsRiConfirmedBalance**](GetAddressDetailsRI_confirmedBalance.md) |  | 
**total_received** | [**crate::models::GetAddressDetailsRiTotalReceived**](GetAddressDetailsRI_totalReceived.md) |  | 
**total_spent** | [**crate::models::GetAddressDetailsRiTotalSpent**](GetAddressDetailsRI_totalSpent.md) |  | 
**incoming_transactions_count** | **i32** | Numeric representation of the received transaction count of the address | 
**outgoing_transactions_count** | **i32** | Numeric representation of the sent transaction count of the address | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


