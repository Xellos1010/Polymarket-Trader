# set usdc conversion

```
curl --request POST \
  --url https://api.exchange.coinbase.com/margin/usdc \
  --header 'Content-Type: application/json' \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>' \
  --data '
{
  "enabled": true
}
'
```

Set the USDC to USD auto conversion to meet margin calls with eligible Coinbase Affiliate

```
curl --request POST \
  --url https://api.exchange.coinbase.com/margin/usdc \
  --header 'Content-Type: application/json' \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>' \
  --data '
{
  "enabled": true
}
'
```

#### Authorizations

#### Body

#### Response