# deposit fiat

## Table of Endpoints

Name

Method

Endpoint

Legacy Scope

CDP API Key Scope

[Deposit Funds](#deposit-funds)

POST

`/v2/accounts/:account_id/deposits`

`wallet:deposits:create`

`transfer`

[Commit Deposit](#commit-deposit)

POST

`/v2/accounts/:account_id/deposits/:deposit_id/commit`

`wallet:deposits:create`

`transfer`

[List Deposits](#list-deposits)

GET

`/v2/accounts/:account_id/deposits`

`wallet:deposits:read`

`view`

[Show Deposit](#show-deposit)

GET

`/v2/accounts/:account_id/deposits/:deposit_id`

`wallet:deposits:read`

`view`

## Overview

The **Deposit resource** represents a deposit of funds using a payment method (e.g., a bank). Each committed deposit also has an associated transaction.

Parameter

Description

`id` *string*

Resource ID

`status` *string, enumerable*

Status of the deposit. Valid values: `created`, `completed`, `canceled`

`payment_method` *hash*

Associated payment method (e.g., a bank)

`transaction` *hash*

Associated transaction (e.g., a bank, fiat account)

`amount` *money hash*

Amount

`subtotal` *money hash*

Amount without fees

`fee` *money hash*

Fees associated to this deposit

`created_at` *timestamp*

`updated_at` *timestamp*

`resource` *string, constant **deposit***

`resource_path` *string*

`committed` *boolean*

Has this deposit been committed?

`payout_at` *timestamp, optional*

When a deposit isn’t executed instantly, it receives a payout date for the time it will be executed

#### Example Deposit Resource

```
{
  "id": "67e0eaec-07d7-54c4-a72c-2e92826897df",
  "status": "completed",
  "payment_method": {
    "id": "83562370-3e5c-51db-87da-752af5ab9559",
    "resource": "payment_method",
    "resource_path": "/v2/payment-methods/83562370-3e5c-51db-87da-752af5ab9559"
  },
  "transaction": {
    "id": "441b9494-b3f0-5b98-b9b0-4d82c21c252a",
    "resource": "transaction",
    "resource_path": "/v2/accounts/2bbf394c-193b-5b2a-9155-3b4732659ede/transactions/441b9494-b3f0-5b98-b9b0-4d82c21c252a"
  },
  "amount": {
    "amount": "10.00",
    "currency": "USD"
  },
  "subtotal": {
    "amount": "10.00",
    "currency": "USD"
  },
  "created_at": "2015-01-31T20:49:02Z",
  "updated_at": "2015-02-11T16:54:02-08:00",
  "resource": "deposit",
  "resource_path": "/v2/accounts/2bbf394c-193b-5b2a-9155-3b4732659ede/deposits/67e0eaec-07d7-54c4-a72c-2e92826897df",
  "committed": true,
  "fee": {
    "amount": "0.00",
    "currency": "USD"
  },
  "payout_at": "2015-02-18T16:54:00-08:00"
}

```

## Deposit Funds

Deposits user-defined amount of funds to a fiat account.

### HTTP Request

`POST https://api.coinbase.com/v2/accounts/:account_id/deposits`

### Scopes

-   `wallet:deposits:create`

### Arguments

Parameter

Type

Required

Description

`amount`

string

Required

Deposit amount

`currency`

string

Required

Currency for the `amount`

`payment_method`

string

Required

ID of payment method to be used for the deposit. List Payment Methods: `GET /payment-methods`

`commit`

boolean

Optional

If `false`, this deposit is not immediately completed. Use the `commit` call to complete it. Default value: `false`

### Examples

#### Request

#### Response (200)

```
{
  {
    "transfer": {
        "user_entered_amount": {
            "value": "20",
            "currency": "USD"
        },
        "amount": {
            "value": "20",
            "currency": "USD"
        },
        "total": {
            "value": "20",
            "currency": "USD"
        },
        "subtotal": {
            "value": "20",
            "currency": "USD"
        },
        "idem": "7ada05f0-4ab9-4e42-8cb9-4501e795315d",
        "committed": false,
        "id": "7ada05f0-4ab9-4e42-8cb9-4501e795315d",
        "instant": true,
        "source": {
            "type": "EXTERNAL_PAYMENT_METHOD",
            "network": "ach",
            "payment_method_id": "",
            "external_payment_method": {
                "payment_method_id": "5a48fe239b15170130598e9c"
            }
        },
        "target": {
            "type": "LEDGER_ACCOUNT",
            "network": "internal_retail",
            "payment_method_id": "",
            "ledger_account": {
                "account_id": "6c770048-a3aa-580b-b153-2a6791649ee4",
                "currency": "USD",
                "owner": {
                    "id": "5a48fda3bbf66c03a6509af2",
                    "uuid": "",
                    "user_uuid": "",
                    "type": "RETAIL"
                }
            }
        },
        "payout_at": "2025-04-09T20:13:48.917581730Z",
        "status": "",
        "user_reference": "",
        "type": "TRANSFER_TYPE_DEPOSIT",
        "created_at": null,
        "updated_at": null,
        "user_warnings": [],
        "fees": [],
        "total_fee": {
            "title": "Fee Total",
            "description": "Total fee associated with this transaction",
            "amount": {
                "value": "0.00",
                "currency": "USD"
            },
            "type": "COINBASE"
        },
        "cancellation_reason": null,
        "hold_days": 0,
        "nextStep": null,
        "checkout_url": "",
        "requires_completion_step": false
    }
}
}

```

## Commit Deposit

Completes a [deposit](#deposit-funds) that is created in `commit: false` state.

### HTTP Request

`POST https://api.coinbase.com/v2/accounts/:account_id/deposits/:deposit_id/commit`

### Scopes

-   `wallet:deposits:create`

### Arguments

*None*

### Examples

#### Request

#### Response (200)

```
{
    "transfer": {
        "user_entered_amount": {
            "value": "20",
            "currency": "USD"
        },
        "amount": {
            "value": "20",
            "currency": "USD"
        },
        "total": {
            "value": "20",
            "currency": "USD"
        },
        "subtotal": {
            "value": "20",
            "currency": "USD"
        },
        "idem": "bd4d2728-9d0c-478e-829e-8f4b4888b108",
        "committed": false,
        "id": "bd4d2728-9d0c-478e-829e-8f4b4888b108",
        "instant": true,
        "source": {
            "type": "EXTERNAL_PAYMENT_METHOD",
            "network": "ach",
            "payment_method_id": "",
            "external_payment_method": {
                "payment_method_id": "5a48fe239b15170130598e9c"
            }
        },
        "target": {
            "type": "LEDGER_ACCOUNT",
            "network": "internal_retail",
            "payment_method_id": "",
            "ledger_account": {
                "account_id": "6c770048-a3aa-580b-b153-2a6791649ee4",
                "currency": "USD",
                "owner": {
                    "id": "5a48fda3bbf66c03a6509af2",
                    "uuid": "",
                    "user_uuid": "",
                    "type": "RETAIL"
                }
            }
        },
        "payout_at": "2025-04-10T18:07:25.938533583Z",
        "status": "",
        "user_reference": "CODE",
        "type": "TRANSFER_TYPE_DEPOSIT",
        "created_at": null,
        "updated_at": null,
        "user_warnings": [],
        "fees": [],
        "total_fee": {
            "title": "Fee Total",
            "description": "Total fee associated with this transaction",
            "amount": {
                "value": "0.00",
                "currency": "USD"
            },
            "type": "COINBASE"
        },
        "cancellation_reason": null,
        "hold_days": 0,
        "nextStep": null,
        "checkout_url": "",
        "requires_completion_step": false
    }
}

```

## List Deposits

Lists fiat deposits for an account.

### HTTP Request

`GET https://api.coinbase.com/v2/accounts/:account_id/deposits`

### Scopes

-   `wallet:deposits:read`

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
      "id": "67e0eaec-07d7-54c4-a72c-2e92826897df",
      "status": "completed",
      "payment_method": {
        "id": "83562370-3e5c-51db-87da-752af5ab9559",
        "resource": "payment_method",
        "resource_path": "/v2/payment-methods/83562370-3e5c-51db-87da-752af5ab9559"
      },
      "transaction": {
        "id": "441b9494-b3f0-5b98-b9b0-4d82c21c252a",
        "resource": "transaction",
        "resource_path": "/v2/accounts/2bbf394c-193b-5b2a-9155-3b4732659ede/transactions/441b9494-b3f0-5b98-b9b0-4d82c21c252a"
      },
      "amount": {
        "amount": "10.00",
        "currency": "USD"
      },
      "subtotal": {
        "amount": "10.00",
        "currency": "USD"
      },
      "created_at": "2015-01-31T20:49:02Z",
      "updated_at": "2015-02-11T16:54:02-08:00",
      "resource": "deposit",
      "resource_path": "/v2/accounts/2bbf394c-193b-5b2a-9155-3b4732659ede/deposits/67e0eaec-07d7-54c4-a72c-2e92826897df",
      "committed": true,
      "fee": {
        "amount": "0.00",
        "currency": "USD"
      },
      "payout_at": "2015-02-18T16:54:00-08:00"
    }
  ]
}

```

## Show Deposit

Get one deposit by deposit Id.

### HTTP Request

`GET https://api.coinbase.com/v2/accounts/:account_id/deposits/:deposit_id`

### Scopes

-   `wallet:deposits:read`

### Examples

#### Request

#### Response

```
{
  "data": {
    "id": "67e0eaec-07d7-54c4-a72c-2e92826897df",
    "status": "completed",
    "payment_method": {
      "id": "83562370-3e5c-51db-87da-752af5ab9559",
      "resource": "payment_method",
      "resource_path": "/v2/payment-methods/83562370-3e5c-51db-87da-752af5ab9559"
    },
    "transaction": {
      "id": "441b9494-b3f0-5b98-b9b0-4d82c21c252a",
      "resource": "transaction",
      "resource_path": "/v2/accounts/2bbf394c-193b-5b2a-9155-3b4732659ede/transactions/441b9494-b3f0-5b98-b9b0-4d82c21c252a"
    },
    "amount": {
      "amount": "10.00",
      "currency": "USD"
    },
    "subtotal": {
      "amount": "10.00",
      "currency": "USD"
    },
    "created_at": "2015-01-31T20:49:02Z",
    "updated_at": "2015-02-11T16:54:02-08:00",
    "resource": "deposit",
    "resource_path": "/v2/accounts/2bbf394c-193b-5b2a-9155-3b4732659ede/deposits/67e0eaec-07d7-54c4-a72c-2e92826897df",
    "committed": true,
    "fee": {
      "amount": "0.00",
      "currency": "USD"
    },
    "payout_at": "2015-02-18T16:54:00-08:00"
  }
}

```