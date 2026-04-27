# send crypto

## Table of Endpoints

Name

Method

Endpoint

Legacy Scope

Send Money

POST

`/v2/accounts/:account_id/transactions`

`wallet:transactions:send`

## Send Money

Send funds to a network address for any [Coinbase supported asset](https://help.coinbase.com/en/coinbase/supported-crypto), or email address of the recipient. No transaction fees are required for off-blockchain cryptocurrency transactions.

The Send money API is asynchronous, which means that Coinbase may delay or cancel the send when necessary. Coinbase recommends that you poll the `status` field for the `completed` state with the [Show a Transaction API](https://developer.chrome.com/coinbase-app/track-apis/transactions). You can also see if a transaction is `pending` in the Send Money API response. When used with OAuth2 authentication, this endpoint requires [two factor authentication](https://developer.chrome.com/coinbase-app/oauth2-integration/2fa).

* * *

## HTTP Request

```
POST https://api.coinbase.com/v2/accounts/:account_id/transactions

```

* * *

## Scopes

-   `wallet:transactions:send`

* * *

## Arguments

Parameter

Type

Required

Description

`type`, constant `send`

string

Required

Type `send` is required when sending money

`to`

string

Required

A blockchain address, or email of the recipient

`amount`

string

Required

Amount to be sent

`currency`

string

Required

Currency of the `amount`

`description`

string

Optional

Notes to be included in the email to the recipient

`skip_notifications`

boolean

Optional

Don’t send notification emails for small amounts (e.g., tips)

`idem`

string

Optional

**\[Recommended\]** A UUIDv4 token to ensure [idempotence](https://en.wikipedia.org/wiki/Idempotence).

`destination_tag`

string

Optional

For select currencies, `destination_tag` or `memo` indicates the beneficiary or destination

`network`

string

Optional

Network to be sent on for a blockchain address send. Defaults to the currency’s default network.

`travel_rule_data`

object

Optional

Visit the [Guide](https://developers.coinbase.com/docs/wallet/guides/travel-rule) for more details

* * *

## Examples

### Request (Shell)

```
curl https://api.coinbase.com/v2/accounts/2bbf394c-193b-5b2a-9155-3b4732699ede/transactions \
  -X POST \
  -H 'Content-Type: application/json' \
  -H 'Authorization: Bearer ab09d9f5f27a7b170cd775abf89632b35b07c1e9d53e08b340cd9832ce52c2c' \
  -d '{
    "type": "send",
    "to": "1AUJ8z5RuHkTPQ1eikyfUUetzGmdwLGkpT",
    "amount": "0.1",
    "currency": "BTC",
    "idem": "df087dce-92a8-45cf-b112-60aad22c0976",
    "network": "bitcoin"
}'

```

* * *

### Response (201)

```
{
  "data": {
    "id": "3c04e35e-8e5a-5ff1-9155-00675db4ac02",
    "type": "send",
    "status": "pending",
    "amount": {
      "amount": "-0.10000000",
      "currency": "BTC"
    },
    "native_amount": {
      "amount": "-1.00",
      "currency": "USD"
    },
    "description": null,
    "created_at": "2015-01-31T20:49:02Z",
    "updated_at": "2015-03-31T17:25:29-07:00",
    "resource": "transaction",
    "resource_path": "/v2/accounts/2bbf394c-193b-5b2a-9155-3b4732699ede/transactions/3c04e35e-8e5a-5ff1-9155-00675db4ac02",
    "network": {
      "status": "pending"
    },
    "to": {
      "resource": "bitcoin_address",
      "address": "1AUJ8z5RuHkTPQ1eikyfUUetzGmdwLGkpT"
    }
  }
}

```