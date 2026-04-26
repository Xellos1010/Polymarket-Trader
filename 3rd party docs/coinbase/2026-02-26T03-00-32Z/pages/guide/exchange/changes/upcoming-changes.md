# upcoming changes

This page provides information about upcoming changes to Coinbase Exchange.

## SL/TPSL Order Cancel Replace Support

*Updated: 2025-Dec-02* *To Be Released: 2025-Dec-04* We are expanding [OrderCancelReplaceRequest (35=G)](https://developer.chrome.com/exchange/fix-api/order-entry-messages/order-entry-messages5#ordercancelreplacerequest-35=g) to support additional order types. Currently, this message only supports limit orders. With this update, you will also be able to modify stop orders and [TPSL (Take Profit/Stop Loss)](https://developer.chrome.com/exchange/fix-api/order-entry-messages/tpsl-orders) orders using the cancel-replace workflow.

## Deleting travel rule fields in POST /withdrawals/crypto REST API

*Updated: 2025-Jan-13* We are removing travel rule fields from `POST /withdrawals/crypto` REST API. Customers in travel rule jurisdictions can withdraw only to their allowlisted addresses.

## Adding new PUT /address-book/{id} REST endpoint

*Added: 2025-Jan-8* We are introducing a new REST endpoint to edit an editing existing address book entry - useful for customers in travel rule jurisdictions. This endpoint requires the API key to have **MANAGE** permissions. Non travel-rule jurisdictions can only edit the label of the address book entry. Example request `PUT https://api.exchange.coinbase.com/address-book/{id}`. Here {id} refers to uuid of the crypto address.

```
{
  "label": "string", // label for crypto address
  "is_certified_self_send": bool // true if customer owns the address/ false if it is a third party address
  "vasp_id": "string" // optional - vasp name from supported list if the wallet address is a VASP address
  "is_verified_self_hosted_wallet": bool // optional - true if the wallet is verified self-hosted wallet
  "business_name": "lorem ipsum", // required for third-party address; ie is_certified_self_send is false
  "business_country_code": "DE" // ISO 3166-1 alpha-2 country code required for third-party address; ie is_certified_self_send is false
}

```

Sample response:

```
{
  "body": {
    "id": "e89b6ea2-1d73-4b3c-9f3a-3d9c8f25b7d9",
    "address": "0x6448894b9499AeebD914232483d0d0467194efcp",
    "label": "string",
    "address_info": {
      "address": "0x6448894b9499AeebD914232483d0d0467194efcp",
      "display_address": "0x6448894b9499AeebD914232483d0d0467194efcp",
      "destination_tag": "string"
    },
    "display_address": "0x6448894b9499AeebD914232483d0d0467194efcp",
    "address_booked": true,
    "address_book_added_at": "2024-03-19T12:00:00Z",
    "address_book_entry_pending_until": "2024-03-21T12:00:00Z",
    "currency": "USDC",
    "is_verified_self_hosted_wallet": false,
    "vasp_id": "string",
    "business_name": "string",
    "business_country_code": "DE"
  }
}

```

note: business name and country code are only populated for travel rule regions.