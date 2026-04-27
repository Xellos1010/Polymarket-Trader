# set auto loan

```
curl --request POST \
  --url https://api.exchange.coinbase.com/margin/auto-loan \
  --header 'Content-Type: application/json' \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>' \
  --data '
{
  "auto_loan": true
}
'
```

Automatically initiate loans to meet margin calls with Coinbase affiliates in accordance with the Lending Agreement and Side Letter

```
curl --request POST \
  --url https://api.exchange.coinbase.com/margin/auto-loan \
  --header 'Content-Type: application/json' \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>' \
  --data '
{
  "auto_loan": true
}
'
```

#### Authorizations

#### Body

#### Response