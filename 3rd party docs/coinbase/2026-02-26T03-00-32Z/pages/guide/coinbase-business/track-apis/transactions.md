# transactions

## Table of Endpoints

Name

Method

Endpoint

Legacy Scope

CDP API Key Scope

[Send Money](#send-money)

POST

`/v2/accounts/:account_id/transactions`

`wallet:transactions:send`

`transfer`

[List Transactions](#list-transactions)

GET

`/v2/accounts/:account_id/transactions`

`wallet:transactions:read`

`view`

[Show Transaction](#show-transaction)

GET

`/v2/accounts/:account_id/transactions/:transaction_id`

`wallet:transactions:read`

`view`

## Overview

The **Transaction resource** represents an event on the account. An `amount` can be positive (credit) or negative (debit). Transactions with counterparties have either a `to` or `from` field.

### Transaction Types

Transaction types available:

Transaction Type

Description

`advanced_trade_fill`

Fills for an advanced trade order

`buy`

Buy a digital asset

`clawback`

Recover money already disbursed

`derivatives_settlement`

Daily cash transfers between futures and spot accounts for the US-regulated futures product

`earn_payout`

Payout for user earn on Coinbase

`fiat_deposit`

Deposit funds into a fiat account from a financial institution

`fiat_withdrawal`

Withdraw funds from a fiat account

`incentives_rewards_payout`

Redemptions for Incentive & Referral campaigns

`incentives_shared_clawback`

Clawback incentive payout from customer account

`intx_deposit`

Deposit crypto to customer international account

`intx_withdrawal`

Withdraw crypto from customer international account

`receive`

Receive a digital asset

`request`

Request a digital asset from a user or email

`retail_simple_dust`

Sweep of dust balance from the account

`sell`

Sell a digital asset

`send`

Send a [supported digital asset](https://www.coinbase.com/trade) to a corresponding address or email.

Note: Previously functioned as a default catch-all type. Now it is restricted to send transactions only.

`staking_transfer`

Funds from primary account moved to staked account

`subscription_rebate`

Transaction for Coinbase subscription rebate

`subscription`

Transaction for Coinbase subscription

`trade`

Exchange one cryptocurrency for another cryptocurrency or fiat currency

`transfer`

Transfer funds between two of your own accounts

`tx`

**Default transaction type, uncategorized.**

`unstaking_transfer`

Funds from staked funds moved to primary account

`unsupported_asset_recovery`

Recover unsupported ERC-20s deposited to Coinbase on ethereum mainnet

`unwrap_asset`

Unwrap wrapped assets, e.g. cbETH, to wrappable assets, e.g. staked ETH

`vault_withdrawal`

Withdraw funds from a vault account

`wrap_asset`

Wrap wrappable assets, e.g. staked ETH, to wrapped assets, e.g. cbETH

`fcm_futures_usdc_sell`

Conversion of USDC to USD to support the anticipated margin requirement for a futures trade

`fcm_futures_usdc_sell_additional_encumberment_rollup`

Conversion of USDC to USD to support additional margin requirements or cover losses for open futures positions

Note: Type `tx` is the default and uncategorized and additional types will be added over time.

### Transaction Statuses

Transaction statuses vary based on the type of the transaction. As both types and statuses can change over time, we recommend that you use `details` field for constructing human readable descriptions of transactions. Currently available statuses are:

Transaction Status

Description

`canceled`

Transaction was canceled

`completed`

Completed transactions (e.g., a send or a buy)

`expired`

Conditional transaction expired due to external factors

`failed`

Failed transactions (e.g., failed buy)

`pending`

Pending transactions (e.g., a send or a buy)

`waiting_for_clearing`

Vault withdrawal is waiting to be cleared

`waiting_for_signature`

Vault withdrawal is waiting for approval

### Parameters

Parameter

Type

Required

Description

`id`

string

Required

Transaction ID

`type`

string, enum

Required

Transaction type

`status`

string, enum

Required

Status

`amount`

money hash

Required

Amount of any [supported digital asset](https://www.coinbase.com/trade). Value is negative to indicate the debiting of funds for the following transaction type cases:

-   `advanced_trade_fill`, sell
-   `pro_deposit`

`native_amount`

money hash

Required

Amount in user’s native currency. Value is negative to indicate the debiting of funds for the following transaction type cases:

-   `advanced_trade_fill`, sell
-   `pro_deposit`

`description`

string

Required

User defined description

`created_at`

timestamp

Required

`updated_at`

timestamp

Required

**Removed for List/Show Tx on 7 Feb**

`resource`, *constant **transaction***

string

Required

`resource_path`

string

Required

`instant_exchange`

boolean

Required

**Removed for List/Show Tx on 7 Feb**

[`advanced_trade_fill`](#advanced-trade-fill)

hash

Required

Only provided if the transaction type is `advanced_trade_fill`. Contains information about the fill posted.

[`details`](#details)

hash

Required

**Removed for List/Show Tx on 7 Feb**  
Detailed information about the transaction

[`network`](#network)

hash

Optional

Info about crypto networks including on-chain transaction hashes. Only available for certain types of transactions.

`to`

hash

Optional

Receiving party of a debit transaction. Usually another resource but can be another type like email. Only available for certain types of transactions.

[`from`](#from)

hash

Optional

Originating party of a credit transaction. Usually another resource, but can be another type like Bitcoin network. Only available for certain types of transactions.

`address`

hash

Optional

**Removed for List/Show Tx on 7 Feb**  
Associated crypto address for received payment

`application`

hash

Optional

**Removed for List/Show Tx on 7 Feb**  
Associated OAuth2 application

`cancelable`

boolean

Optional

**New for List/Show Tx on 7 Feb**  
Allowed to cancel transaction; ONLY provided when transaction is a SEND

`idem`

string

Optional

**New for List/Show Tx on 7 Feb**  
Idempotency key of transaction; ONLY provided when transaction is a SEND

`buy`

hash

Optional

**New for List/Show Tx on 7 Feb**  
Only provided if transaction type is a buy

`sell`

hash

Optional

**New for List/Show Tx on 7 Feb**  
Only provided if transaction type is a sell

`trade`

hash

Optional

**New for List/Show Tx on 7 Feb**  
Only provided if transaction type is a trade

### Advanced Trade Fill

Parameter

Type

Description

`fill_price`

string

Price this fill was posted at

`product_id`

string

`order_id`

string

The UUID of the order this fill belongs to

`commission`

Commission per fill of the order. Always represented in quote currency.

`order_side`

string, enum

Side the order was placed on. Possible values: `BUY`, `SELL`

### Details

Parameter

Type

Description

`title`

string

Description of transaction with currency. Example: “Received Bitcoin”

`subsidebar_label`

string or null

`header`

string

Amount received, in amount and equivalent native amount. Example: “Received 0.005378 BTC ($49.92)“

`health`

string

Health of transaction. Example: “Positive”

### Network

Parameter

Type

Description

`status`

string, enum

Possible values:

-   `off_blockchain`
-   `confirmed`
-   `pending`
-   `unconfirmed`

`status_description`

string or null

**Removed for List/Show Tx on 7 Feb**  
Description of status

`hash`

string

**New for List/Show Tx on 7 Feb**  
Hash for onchain transactions; ONLY provided when transaction is a SEND

`transaction_fee`

hash

**New for List/Show Tx on 7 Feb**  
Transaction fee; ONLY provided when transaction is a SEND

`network_name`

string

**New for List/Show Tx on 7 Feb**  
Name of transaction network; ONLY provided when transaction is a SEND

### From

Parameter

Type

Description

`id`

string

**Updated for List/Show Tx on 7 Feb**  
UUID (and after Feb 7, account) of user who the transaction is from

`resource`

string or null

**Updated for List/Show Tx on 7 Feb**  
user (and after Feb 7, account)

`resource_path`

string

**Updated for List/Show Tx on 7 Feb**  

-   If resource is user, this path is in the form of v2/users/{userUUID}
-   (after Feb 7) If resource is account, the path is in the form of /v2/accounts/{accountUUID}>

`currency`

string

**Removed for List/Show Tx on 7 Feb**  
Currency user sent

### Resource Examples

#### Transaction Resource (Send)

```
{
  "id": "57ffb4ae-0c59-5430-bcd3-3f98f797a66c",
  "type": "send",
  "status": "pending",
  "amount": {
    "amount": "-0.00133",
    "currency": "BTC"
  },
  "native_amount": {
    "amount": "-0.01",
    "currency": "USD"
  },
  "description": null,
  "created_at": "2015-03-11T13:13:35-07:00",
  "updated_at": "2015-03-26T15:55:43-07:00",
  "resource": "transaction",
  "resource_path": "/v2/accounts/2bbf394c-193b-5b2a-9155-3b4732659ede/transactions/57ffb4ae-0c59-5430-bcd3-3f98f797a66c",
  "idem": "df087dce-92a8-45cf-b112-60aad22c0976",
  "network": {
    "status": "pending",
	"transaction_fee": {
	  "amount": "0.00033",
	  "currency": "BTC"
	},
    "transaction_amount": {
	  "amount": "0.001",
	  "currency": "ETH"
    }
  },
  "to": {
    "resource": "ethereum_address",
    "address": "0xabc"
  }
}

```

#### Transaction Resource (Buy)

```
{
  "id": "8250fe29-f5ef-5fc5-8302-0fbacf6be51e",
  "type": "buy",
  "status": "pending",
  "amount": {
    "amount": "1.00000000",
    "currency": "BTC"
  },
  "native_amount": {
    "amount": "10.00",
    "currency": "USD"
  },
  "description": null,
  "created_at": "2015-03-26T13:42:00-07:00",
  "updated_at": "2015-03-26T15:55:45-07:00",
  "resource": "transaction",
  "resource_path": "/v2/accounts/2bbf394c-193b-5b2a-9155-3b4732659ede/transactions/8250fe29-f5ef-5fc5-8302-0fbacf6be51e",
  "details": {
    "title": "Bought bitcoin",
    "subtitle": "using Capital One Bank"
  }
}

```

## Send Money

Send funds to a network address for any [Coinbase supported asset](https://help.coinbase.com/en/coinbase/supported-crypto), or email address of the recipient. No transaction fees are required for off-blockchain cryptocurrency transactions.

The Send money API is asynchronous, which means that Coinbase may delay or cancel the send when necessary. Coinbase recommends that you poll the `status` field for the `completed` state with the [Show a Transaction](#show-a-transaction) API. You can also see if a transaction is `pending` in the Send Money API response. When used with OAuth2 authentication, this endpoint requires [two factor authentication](https://developer.chrome.com/coinbase-app/authentication-authorization/oauth2/2fa).

### HTTP Request

`POST https://api.coinbase.com/v2/accounts/:account_id/transactions`

### Scopes

-   `wallet:transactions:send`

### Arguments

Parameter

Type

Required

Description

`type`, *constant **send***

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

**\[Recommended\]** A UUIDv4 token to ensure [idempotence](http://en.wikipedia.org/wiki/Idempotence). If a previous transaction with the same `idem` parameter exists for this sender, that previous transaction is returned and a new one is *not* created. Max length is 100 characters. Must be a valid UUID and lowercased.

`destination_tag`

string

Optional

For select currencies, `destination_tag` or `memo` indicates the beneficiary or destination of a payment for select currencies. Example: `{ "type" : "send", "to": "address", "destination_tag" : "memo", "amount": "", "currency": "" }`

`network`

string

Optional

Network to be sent on for a blockchain address send. If not specified, send will be executed on the default network for that currency. Ex: “ethereum”, “polygon”, etc.

`travel_rule_data`

object

Optional

Visit the [Guide](https://developer.chrome.com/coinbase-app/transfer-apis/travel-rule) for more details

### Examples

#### Request

-   Shell
    
-   Ruby
    
-   Python
    
-   JavaScript
    

```
curl https://api.coinbase.com/v2/accounts/2bbf394c-193b-5b2a-9155-3b4732659ede/transactions /
  -X POST \
  -H 'Content-Type: application/json' \
  -H 'Authorization: Bearer abd90df5f27a7b170cd775abf89d632b350b7c1c9d53e08b340cd9832ce52c2c' \
  -d '{
    "type": "send",
    "to": "1AUJ8z5RuHRTqD1eikyfUUetzGmdWLGkpT",
    "amount": "0.1",
    "currency": "BTC",
    "idem": "df087dce-92a8-45cf-b112-60aad22c0976",
    "network": "bitcoin",
  }'

```

```
require 'coinbase/wallet'
client = Coinbase::Wallet::Client.new(api_key: <api key>, api_secret: <api secret>)
tx = client.send('2bbf394c-193b-5b2a-9155-3b4732659ede',
                 {'to' => '1AUJ8z5RuHRTqD1eikyfUUetzGmdWLGkpT',
                  'amount' => '0.1',
                  'currency' => 'BTC',
                  'idem' => 'df087dce-92a8-45cf-b112-60aad22c0976'})

```

```
import requests
# For instructions generating JWT, check the "API Key Authentication" section
JWT_TOKEN = "<your_jwt_token>"
ENDPOINT_URL = "https://api.coinbase.com/v2/accounts/:account_id/transactions"
def send_money():
    # Generate headers with JWT for authentication
    headers = {
        "Authorization": f"Bearer {JWT_TOKEN}"
    }
    data = {
      "type": "send",
      "to": "1AUJ8z5RuHRTqD1eikyfUUetzGmdWLGkpT",
      "amount": "0.1",
      "currency": "BTC",
      "idem": "df087dce-92a8-45cf-b112-60aad22c0976",
      "network": "bitcoin",
    }
    # Make the API request
    response = requests.post(ENDPOINT_URL, data=data, headers=headers)
    return response.json()  # Return the JSON response
send_money = send_money()
print(send_money)

```

```
var Client = require('coinbase').Client;
var client = new Client({'apiKey': 'API KEY',
                         'apiSecret': 'API SECRET'});
client.getAccount('2bbf394c-193b-5b2a-9155-3b4732659ede', function(err, account) {
  account.sendMoney({'to': '1AUJ8z5RuHRTqD1eikyfUUetzGmdWLGkpT',
                     'amount': '0.1',
                     'currency': 'BTC',
                     'idem': 'df087dce-92a8-45cf-b112-60aad22c0976'}, function(err, tx) {
    console.log(tx);
  });
});

```

#### Response (201)

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
    "resource_path": "/v2/accounts/2bbf394c-193b-5b2a-9155-3b4732659ede/transactions/3c04e35e-8e5a-5ff1-9155-00675db4ac02",
    "network": {
      "status": "pending",
    },
    "to": {
      "resource": "bitcoin_address",
      "address": "1AUJ8z5RuHRTqD1eikyfUUetzGmdWLGkpT"
    }
  }
}

```

Lists the transactions of an account by account ID.

### HTTP Request

`GET https://api.coinbase.com/v2/accounts/:account_id/transactions`

### Scopes

-   `wallet:transactions:read`

### Examples

#### Request

-   Shell
    
-   Ruby
    
-   Python
    
-   JavaScript
    

```
curl https://api.coinbase.com/v2/accounts/2bbf394c-193b-5b2a-9155-3b4732659ede/transactions /
  -H 'Authorization: Bearer abd90df5f27a7b170cd775abf89d632b350b7c1c9d53e08b340cd9832ce52c2c'

```

```
require 'coinbase/wallet'
client = Coinbase::Wallet::Client.new(api_key: <api key>, api_secret: <api secret>)
txs = client.transactions('2bbf394c-193b-5b2a-9155-3b4732659ede')

```

```
import requests
# For instructions generating JWT, check the "API Key Authentication" section
JWT_TOKEN = "<your_jwt_token>"
ENDPOINT_URL = "https://api.coinbase.com/v2/accounts/:account_id/transactions"
def list_transactions():
    # Generate headers with JWT for authentication
    headers = {
        "Authorization": f"Bearer {JWT_TOKEN}"
    }
    # Make the API request
    response = requests.get(ENDPOINT_URL, headers=headers)
    return response.json()  # Return the JSON response
list_transactions = list_transactions()
print(list_transactions)

```

```
var Client = require('coinbase').Client;
var client = new Client({'apiKey': 'API KEY',
                         'apiSecret': 'API SECRET'});
client.getAccount('2bbf394c-193b-5b2a-9155-3b4732659ede', function(err, account) {
  account.getTransactions(function(err, txs) {
    console.log(txs);
  });
});

```

#### Response (200)

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
      "id": "4117f7d6-5694-5b36-bc8f-847509850ea4",
      "type": "buy",
      "status": "pending",
      "amount": {
        "amount": "486.34313725",
        "currency": "BTC"
      },
      "native_amount": {
        "amount": "4863.43",
        "currency": "USD"
      },
      "description": null,
      "created_at": "2015-03-26T23:44:08-07:00",
      "updated_at": "2015-03-26T23:44:08-07:00",
      "resource": "transaction",
      "resource_path": "/v2/accounts/2bbf394c-193b-5b2a-9155-3b4732659ede/transactions/4117f7d6-5694-5b36-bc8f-847509850ea4",
      "details": {
        "title": "Bought bitcoin",
        "subtitle": "using Capital One Bank"
      }
    },
    {
      "id": "005e55d1-f23a-5d1e-80a4-72943682c055",
      "type": "request",
      "status": "pending",
      "amount": {
        "amount": "0.10000000",
        "currency": "BTC"
      },
      "native_amount": {
        "amount": "1.00",
        "currency": "USD"
      },
      "description": "",
      "created_at": "2015-03-24T18:32:35-07:00",
      "updated_at": "2015-01-31T20:49:02Z",
      "resource": "transaction",
      "resource_path": "/v2/accounts/2bbf394c-193b-5b2a-9155-3b4732659ede/transactions/005e55d1-f23a-5d1e-80a4-72943682c055",
      "to": {
        "resource": "email",
        "email": "rb@coinbase.com"
      },
      "details": {
        "title": "Requested bitcoin",
        "subtitle": "from rb@coinbase.com"
      }
    },
    {
      "id": "ff01bbc6-c4ad-59e1-9601-e87b5b709458",
      "type": "transfer",
      "status": "completed",
      "amount": {
        "amount": "-5.00000000",
        "currency": "BTC"
      },
      "native_amount": {
        "amount": "-50.00",
        "currency": "USD"
      },
      "description": "",
      "created_at": "2015-03-12T15:51:38-07:00",
      "updated_at": "2015-01-31T20:49:02Z",
      "resource": "transaction",
      "resource_path": "/v2/accounts/2bbf394c-193b-5b2a-9155-3b4732659ede/transactions/ff01bbc6-c4ad-59e1-9601-e87b5b709458",
      "to": {
        "id": "58542935-67b5-56e1-a3f9-42686e07fa40",
        "resource": "account",
        "resource_path": "/v2/accounts/58542935-67b5-56e1-a3f9-42686e07fa40"
      },
      "details": {
        "title": "Transferred bitcoin",
        "subtitle": "to Secondary Account"
      }
    },
    {
      "id": "57ffb4ae-0c59-5430-bcd3-3f98f797a66c",
      "type": "send",
      "status": "completed",
      "amount": {
        "amount": "-0.00100000",
        "currency": "BTC"
      },
      "native_amount": {
        "amount": "-0.01",
        "currency": "USD"
      },
      "description": null,
      "created_at": "2015-03-11T13:13:35-07:00",
      "updated_at": "2015-03-26T15:55:43-07:00",
      "resource": "transaction",
      "resource_path": "/v2/accounts/2bbf394c-193b-5b2a-9155-3b4732659ede/transactions/57ffb4ae-0c59-5430-bcd3-3f98f797a66c",
      "network": {
        "status": "off_blockchain",
        "name": "bitcoin"
      },
      "to": {
        "id": "a6b4c2df-a62c-5d68-822a-dd4e2102e703",
        "resource": "user",
        "resource_path": "/v2/users/a6b4c2df-a62c-5d68-822a-dd4e2102e703"
      },
      "details": {
        "title": "Send bitcoin",
        "subtitle": "to User 2"
      }
    }
  ]
}

```

## Show Transaction

Get a single transaction for an account.

### HTTP Request

`GET https://api.coinbase.com/v2/accounts/:account_id/transactions/:transaction_id`

### Scopes

-   `wallet:transactions:read`

### Examples

#### Request

-   Shell
    
-   Ruby
    
-   Python
    
-   JavaScript
    

```
curl https://api.coinbase.com/v2/accounts/2bbf394c-193b-5b2a-9155-3b4732659ede/transactions/57ffb4ae-0c59-5430-bcd3-3f98f797a66c /
  -H 'Authorization: Bearer abd90df5f27a7b170cd775abf89d632b350b7c1c9d53e08b340cd9832ce52c2c'

```

```
require 'coinbase/wallet'
client = Coinbase::Wallet::Client.new(api_key: <api key>, api_secret: <api secret>)
tx = client.transaction('2bbf394c-193b-5b2a-9155-3b4732659ede', '57ffb4ae-0c59-5430-bcd3-3f98f797a66c')

```

```
import requests
# For instructions generating JWT, check the "API Key Authentication" section
JWT_TOKEN = "<your_jwt_token>"
# Coinbase API base URL
ENDPOINT_URL = "https://api.coinbase.com/v2/accounts/:account_id/transactions/:transaction_id"
def show_transaction():
    # Generate headers with JWT for authentication
    headers = {
        "Authorization": f"Bearer {JWT_TOKEN}"
    }
    # Make the API request
    response = requests.get(ENDPOINT_URL, headers=headers)
    return response.json()  # Return the JSON response
transaction = show_transaction()
print(transaction)

```

```
var Client = require('coinbase').Client;
var client = new Client({'apiKey': 'API KEY',
                         'apiSecret': 'API SECRET'});
client.getAccount('2bbf394c-193b-5b2a-9155-3b4732659ede', function(err, account) {
  account.getTransaction('57ffb4ae-0c59-5430-bcd3-3f98f797a66c', function(err, tx) {
    console.log(tx);
  });
});

```

#### Response (200)

```
{
  "data": {
    "id": "57ffb4ae-0c59-5430-bcd3-3f98f797a66c",
    "type": "send",
    "status": "completed",
    "amount": {
      "amount": "-0.00100000",
      "currency": "BTC"
    },
    "native_amount": {
      "amount": "-0.01",
      "currency": "USD"
    },
    "description": null,
    "created_at": "2015-03-11T13:13:35-07:00",
    "updated_at": "2015-03-26T15:55:43-07:00",
    "resource": "transaction",
    "resource_path": "/v2/accounts/2bbf394c-193b-5b2a-9155-3b4732659ede/transactions/57ffb4ae-0c59-5430-bcd3-3f98f797a66c",
    "network": {
      "status": "off_blockchain",
      "name": "bitcoin"
    },
    "to": {
      "id": "a6b4c2df-a62c-5d68-822a-dd4e2102e703",
      "resource": "user",
      "resource_path": "/v2/users/a6b4c2df-a62c-5d68-822a-dd4e2102e703"
    },
    "details": {
      "title": "Send bitcoin",
      "subtitle": "to User 2"
    }
  }
}

```