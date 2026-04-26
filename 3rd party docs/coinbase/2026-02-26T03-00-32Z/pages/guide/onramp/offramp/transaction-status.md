# transaction status

## Offramp Transaction Status

The Offramp Transaction Status API provides developers with a list of user Offramp transactions. Developers can poll the real time status of offramp transactions and show users a view of their Coinbase transactions made within the client app. To link all transactions created during the session, developers must provide the (optional) field `partnerUserRef` as a query parameter when initializing Offramp. Transaction Status returns a paginated list of all transactions from newest to oldest. If the client app doesn’t have a concept of a user, clients can pass a random `partnerUserRef` to reference a one-off session. There are two ways to get the transaction status:

1.  Use the [fetchOnrampTransactionStatus](https://docs.base.org/builderkits/onchainkit/fund/fetch-onramp-transaction-status#fetchonramptransactionstatus) util to get the transaction status
2.  Make a direct call to the API

### Method

### URL

```
https://api.developer.coinbase.com/onramp/v1/sell/user/{partner_user_ref}/transactions?page_key={next_page_key}&page_size={page_size}

```

### Request Parameters

The Transaction Status API is an RPC endpoint that accepts an argument as part of its URL path.

Name

Type

Req

Description

`partner_user_ref`

String

Y

ID referring to user Offramp transactions in client app.

`page_key`

String

N

Reference to next page of transactions. Returned in previous page’s response.

`page_size`

Number

N

Number of transactions to return per page. Default is 1.

### Response Fields

The Transaction Status API returns a JSON response including the following fields.

Name

Description

`transactions`

List of `OfframpTransactions` in reverse chronological order.

`next_page_key`

A reference to the next page of transactions.

`total_count`

The total number of transactions made by the user.

#### OfframpTransaction Schema

Name

Description

Value

`status`

Status of the offramp transaction.

`TRANSACTION_STATUS_STARTED`  
`TRANSACTION_STATUS_SUCCESS`  
`TRANSACTION_STATUS_FAILED`

`asset`

Crypto currency being sold.

String

`network`

Network that crypto currency is on in the user’s wallet.

String

`sell_amount`

Amount of crypto currency being sold.

String

`total`

Total amount of fiat the user will receive, fees deducted. \*Will be null unless using Offramp to Fiat

String

`subtotal`

Amount of fiat for the crypto asset, fees not deducted. \*Will be null unless using Offramp to Fiat

String

`coinbase_fee`

Amount of fiat charged to cover brokerage fees. \*Will be null unless using Offramp to Fiat

String

`exchange_rate`

Unit price of the crypto currency being sold. \*Will be null unless using Offramp to Fiat

String

`from_address`

The address of the wallet the transaction was sent from.

String

`to_address`

The address of the coinbase address the transaction was sent to.

String

`tx_hash`

The block hash of the onchain send.

String

`payment_method`

Type of payment method used to cash out funds.

`FIAT_WALLET`  
`CRYPTO_WALLET`  
`ACH_BANK_ACCOUNT`  
`PAYPAL`  

### Example Request/Response

-   Request (cURL)
    
-   Response 200 (JSON)
    

```
cdpcurl -k /tmp/cdp_api_key.json 'https://api.developer.coinbase.com/onramp/v1/sell/user/{partner_user_ref}/transactions?page_key={next_page_key}&page_size={page_size}'

```

```
{
    "transactions": [
        {
            "id": "1ef6658f-697e-6200-b081-ba4f2149344a",
            "asset": "ETH",
            "status": "TRANSACTION_STATUS_SUCCESS",
            "network": "base",
            "sell_amount": {
                "value": "0.005",
                "currency": "ETH"
            },
            "total": {
                "value": "12.58",
                "currency": "USD"
            },
            "subtotal": {
                "value": "12.51",
                "currency": "USD"
            },
            "coinbase_fee": {
                "value": "0.07",
                "currency": "USD"
            },
            "exchange_rate": {
                "value": "0.000453720508167",
                "currency": "ETH"
            },
            "fromAddress": "0x7D09655eF4664ED4D449b0d27E15AdA1D2Ae3bE5",
            "toAddress": "0x0b59d1001629c86da136a0B480Db68EDBf70e222",
            "tx_hash": "0x31814ba2fef2a400a9816767370be09ca12d9e9753b972198fe330a0ecedcbf6",
            "created_at": "2024-08-29T20:06:07.076Z",
            "updated_at": "2024-08-29T20:10:24.054Z",
        }
    ],
    "next_page_key":"eyJndF9za2lwIjowLCJvdF9za2lwIjoxfQ==",
    "total_count":"3"
}

```

## Offramp Transactions

The Offramp Transactions API provides clients with a list of historical Offramp transactions between two dates. Transactions returns a paginated list of all transactions from newest to oldest. The Transactions API is indented for analytics purposes. If you need real time information about a specific transaction, use the [Offramp Transaction Status API](#offramp-transaction-status).

### Method

### URL

```
https://api.developer.coinbase.com/onramp/v1/sell/transactions?page_key={next_page_key}&page_size={page_size}&start_date={start_date}&end_date={end_date}

```

### Request Parameters

Name

Type

Req

Description

`page_key`

String

N

Reference to next page of transactions. Returned in previous page’s response.

`page_size`

Number

N

Number of transactions to return per page. Default is 1000.

`start_date`

String

N

The start date (inclusive) of the range of transactions to return. YYYY-MM-DD format - UTC Timezone. Default is one month before `end_date`.

`end_date`

String

N

The end date (exclusive) of the range of transactions to return. YYYY-MM-DD format. Default is tomorrow.

### Response Fields

The Transaction API returns a JSON response including the following fields.

Name

Description

`transactions`

List of `OfframpTransactions` in reverse chronological order.

`next_page_key`

A reference to the next page of transactions.

#### OfframpTransaction Schema

Name

Description

Value

`status`

Status of the sell transaction.

`TRANSACTION_STATUS_SUCCESS`  
`TRANSACTION_STATUS_FAILED`

`asset`

Crypto currency being sold.

String

`network`

Network that crypto currency is on in the user’s wallet.

String

`sell_amount`

Amount of crypto currency being sold.

String

`total`

Total amount of fiat the user will receive, fees deducted.

String

`subtotal`

Amount of fiat for the crypto asset, fees not deducted.

String

`coinbase_fee`

Amount of fiat charged to cover brokerage fees.

String

`exchange_rate`

Unit price of the crypto currency being sold.

String

`from_address`

The address of the wallet the transaction was sent from.

String

`to_address`

The address of the coinbase address the transaction was sent to.

String

`tx_hash`

The block hash of the onchain send.

String

### Example Request/Response

-   Request (cURL)
    
-   Response 200 (JSON)
    

```
cdpcurl -k /tmp/cdp_api_key.json 'https://api.developer.coinbase.com/onramp/v1/sell/transactions?page_key={next_page_key}&page_size={page_size}&start_date={start_date}&end_date={end_date}'

```

```
{
    "transactions": [
        {
            "id": "1ef6658f-697e-6200-b081-ba4f2149344a",
            "asset": "ETH",
            "status": "TRANSACTION_STATUS_SUCCESS",
            "network": "base",
            "sell_amount": {
                "value": "0.005",
                "currency": "ETH"
            },
            "total": {
                "value": "12.58",
                "currency": "USD"
            },
            "subtotal": {
                "value": "12.51",
                "currency": "USD"
            },
            "coinbase_fee": {
                "value": "0.07",
                "currency": "USD"
            },
            "exchange_rate": {
                "value": "0.000453720508167",
                "currency": "ETH"
            },
            "fromAddress": "0x7D09655eF4664ED4D449b0d27E15AdA1D2Ae3bE5",
            "toAddress": "0x0b59d1001629c86da136a0B480Db68EDBf70e222",
            "tx_hash": "0x31814ba2fef2a400a9816767370be09ca12d9e9753b972198fe330a0ecedcbf6",
            "created_at": "2024-08-29T20:06:07.076Z",
            "updated_at": "2024-08-29T20:10:24.054Z",
        }
    ],
    "next_page_key":"eyJndF9za2lwIjowLCJvdF9za2lwIjoxfQ==",
    "total_count":"3"
}

```