# GetAddressDetailsResponseItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transactions_count** | **i32** | Represents the total number of confirmed coins transactions for this address, both incoming and outgoing. Applies for coins only **and not** tokens transfers e.g. for Ethereum. `transactionsCount` could result as less than incoming and outgoing transactions put together (e.g. in Bitcoin), due to the fact that one and the same address could be in senders and receivers addresses. | 
**confirmed_balance** | [**crate::models::GetAddressDetailsResponseItemConfirmedBalance**](GetAddressDetailsResponseItem_confirmedBalance.md) |  | 
**total_received** | [**crate::models::GetAddressDetailsResponseItemTotalReceived**](GetAddressDetailsResponseItem_totalReceived.md) |  | 
**total_spent** | [**crate::models::GetAddressDetailsResponseItemTotalSpent**](GetAddressDetailsResponseItem_totalSpent.md) |  | 
**incoming_transactions_count** | **i32** | Defines the count of all confirmed incoming transactions from the address for coins. This applies to **coins** only, **not** to tokens transfers e.g. for Ethereum. | 
**outgoing_transactions_count** | **i32** | Defines the count of all confirmed outgoing transactions from the address for coins. This applies to **coins** only, **not** to tokens transfers e.g. for Ethereum. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


