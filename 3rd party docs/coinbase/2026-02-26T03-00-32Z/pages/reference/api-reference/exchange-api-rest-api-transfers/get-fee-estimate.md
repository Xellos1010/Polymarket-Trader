# get fee estimate

Get fee estimate for crypto withdrawal

```
curl --request GET \
  --url https://api.exchange.coinbase.com/withdrawals/fee-estimate \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>'
```

```
{
  "fee": "<string>",
  "fee_before_subsidy": "<string>"
}
```

Gets the fee estimate for the crypto withdrawal to crypto address

GET

/

withdrawals

/

fee-estimate

Get fee estimate for crypto withdrawal

```
curl --request GET \
  --url https://api.exchange.coinbase.com/withdrawals/fee-estimate \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>'
```

```
{
  "fee": "<string>",
  "fee_before_subsidy": "<string>"
}
```

#### Authorizations

#### Query Parameters

#### Response