# accounts

## Table of Endpoints

Name

Method

Endpoint

Legacy Scope

CDP API Key Scope

[List Accounts](#list-accounts)

GET

`/v2/accounts`

`wallet:accounts:read`

`view`

[Show Account](#show-account)

GET

`/v2/accounts/:account_id`

`wallet:accounts:read`

`view`

## Overview

The **Account resource** represents all of a user’s accounts, including cryptocurrency wallets, fiat currency accounts, and vaults. This is represented in the `type` field. New types may be added in the future, so make sure this won’t break your implementation. User can only have one primary account and its type can only be `wallet`.

Parameter

Description

`id` *string*

Resource ID

`name` *string*

User or system defined name

`primary` *boolean*

Primary account (or not)

`type` *string, enumerable*

Account’s type. Valid values: `wallet`, `fiat`, `vault`

`currency` *hash*

Account’s currency

`balance` *money hash*

Crypto balance

`created_at` *timestamp*

`updated_at` *timestamp*

`resource` *string, constant **account***

`resource_path` *string*

#### Account Resource

```
{
  "id": "2bbf394c-193b-5b2a-9155-3b4732659ede",
  "name": "My Wallet",
  "primary": true,
  "type": "wallet",
  "currency" : {
      "address_regex" : "^([13][a-km-zA-HJ-NP-Z1-9]{25,34})|^(bc1[qzry9x8gf2tvdw0s3jn54khce6mua7l]([qpzry9x8gf2tvdw0s3jn54khce6mua7l]{38}|[qpzry9x8gf2tvdw0s3jn54khce6mua7l]{58}))$",
      "asset_id" : "5b71fc48-3dd3-540c-809b-f8c94d0e68b5",
      "code" : "BTC",
      "color" : "#F7931A",
      "exponent" : 8,
      "name" : "Bitcoin",
      "slug" : "bitcoin",
      "sort_index" : 100,
      "type" : "crypto"
  }
  "balance": {
      "amount": "39.59000000",
      "currency": "BTC"
  },
  "created_at": "2024-01-31T20:49:02Z",
  "updated_at": "2024-01-31T20:49:02Z",
  "resource": "account",
  "resource_path": "/v2/accounts/2bbf394c-193b-5b2a-9155-3b4732659ede"
}

```

## List Accounts

List a current user’s accounts to which the authentication method has access to.

### HTTP Request

`GET https://api.coinbase.com/v2/accounts`

### Scopes

-   `wallet:accounts:read`

### Examples

#### Request

#### Response

```
{
  "pagination": {
    "ending_before": null,
    "starting_after": null,
    "limit": 25,
    "order": "desc",
    "previous_uri": null,
    "next_uri": null
  },
  "data": [
    {
      "id": "58542935-67b5-56e1-a3f9-42686e07fa40",
      "name": "My Vault",
      "primary": false,
      "type": "vault",
      "currency" : {
         "address_regex" : "^([13][a-km-zA-HJ-NP-Z1-9]{25,34})|^(bc1[qzry9x8gf2tvdw0s3jn54khce6mua7l]([qpzry9x8gf2tvdw0s3jn54khce6mua7l]{38}|[qpzry9x8gf2tvdw0s3jn54khce6mua7l]{58}))$",
         "asset_id" : "5b71fc48-3dd3-540c-809b-f8c94d0e68b5",
         "code" : "BTC",
         "color" : "#F7931A",
         "exponent" : 8,
         "name" : "Bitcoin",
         "slug" : "bitcoin",
         "sort_index" : 100,
         "type" : "crypto"
      }
      "balance": {
        "amount": "4.00000000",
        "currency": "BTC"
      },
      "created_at": "2024-01-31T20:49:02Z",
      "updated_at": "2024-01-31T20:49:02Z",
      "resource": "account",
      "resource_path": "/v2/accounts/58542935-67b5-56e1-a3f9-42686e07fa40",
      "ready": true
    },
    {
      "id": "2bbf394c-193b-5b2a-9155-3b4732659ede",
      "name": "My Wallet",
      "primary": true,
      "type": "wallet",
      "currency" : {
         "address_regex" : "^([13][a-km-zA-HJ-NP-Z1-9]{25,34})|^(bc1[qzry9x8gf2tvdw0s3jn54khce6mua7l]([qpzry9x8gf2tvdw0s3jn54khce6mua7l]{38}|[qpzry9x8gf2tvdw0s3jn54khce6mua7l]{58}))$",
         "asset_id" : "5b71fc48-3dd3-540c-809b-f8c94d0e68b5",
         "code" : "BTC",
         "color" : "#F7931A",
         "exponent" : 8,
         "name" : "Bitcoin",
         "slug" : "bitcoin",
         "sort_index" : 100,
         "type" : "crypto"
      }
      "balance": {
        "amount": "39.59000000",
        "currency": "BTC"
      },
      "created_at": "2024-01-31T20:49:02Z",
      "updated_at": "2024-01-31T20:49:02Z",
      "resource": "account",
      "resource_path": "/v2/accounts/2bbf394c-193b-5b2a-9155-3b4732659ede"
    }
  ]
}

```

## Show Account

Show (or get) a current user’s account. To access the primary account for a given currency, a currency string (e.g., `BTC` or `ETH`) can be used instead of the account ID in the URL.

### HTTP Request

`GET https://api.coinbase.com/v2/accounts/:account_id`

### Scopes

-   `wallet:accounts:read`

### Examples

#### Request

#### Response

```
{
  "data": {
    "id": "2bbf394c-193b-5b2a-9155-3b4732659ede",
    "name": "My Wallet",
    "primary": true,
    "type": "wallet",
      "currency" : {
         "address_regex" : "^([13][a-km-zA-HJ-NP-Z1-9]{25,34})|^(bc1[qzry9x8gf2tvdw0s3jn54khce6mua7l]([qpzry9x8gf2tvdw0s3jn54khce6mua7l]{38}|[qpzry9x8gf2tvdw0s3jn54khce6mua7l]{58}))$",
         "asset_id" : "5b71fc48-3dd3-540c-809b-f8c94d0e68b5",
         "code" : "BTC",
         "color" : "#F7931A",
         "exponent" : 8,
         "name" : "Bitcoin",
         "slug" : "bitcoin",
         "sort_index" : 100,
         "type" : "crypto"
      }
    "balance": {
      "amount": "39.59000000",
      "currency": "BTC"
    },
    "created_at": "2024-01-31T20:49:02Z",
    "updated_at": "2024-01-31T20:49:02Z",
    "resource": "account",
    "resource_path": "/v2/accounts/2bbf394c-193b-5b2a-9155-3b4732659ede"
  }
}

```