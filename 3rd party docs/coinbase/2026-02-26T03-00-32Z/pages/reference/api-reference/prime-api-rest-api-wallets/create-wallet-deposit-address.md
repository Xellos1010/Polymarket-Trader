# create wallet deposit address

Create Wallet Deposit Address

```
curl --request POST \
  --url https://api.prime.coinbase.com/v1/portfolios/{portfolio_id}/wallets/{wallet_id}/addresses \
  --header 'Content-Type: application/json' \
  --data '
{
  "network_id": "ethereum-mainnet"
}
'
```

```
{
  "address": "<string>",
  "account_identifier": "<string>",
  "network": {
    "id": "<string>",
    "type": "<string>"
  }
}
```

Creates a new deposit address for a wallet. Only applicable to wallets that support multiple deposit addresses on a given network

POST

/

v1

/

portfolios

/

{portfolio\_id}

/

wallets

/

{wallet\_id}

/

addresses

Create Wallet Deposit Address

```
curl --request POST \
  --url https://api.prime.coinbase.com/v1/portfolios/{portfolio_id}/wallets/{wallet_id}/addresses \
  --header 'Content-Type: application/json' \
  --data '
{
  "network_id": "ethereum-mainnet"
}
'
```

```
{
  "address": "<string>",
  "account_identifier": "<string>",
  "network": {
    "id": "<string>",
    "type": "<string>"
  }
}
```

#### Path Parameters

The ID of the portfolio that owns the wallet

The wallet ID for which to create the deposit address

#### Body

The network name and type

Example:

`"ethereum-mainnet"`

#### Response

The address on the network

The account identifier (used on some chains to distinguish accounts using the same address)