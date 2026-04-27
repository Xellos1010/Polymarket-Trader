# update settlement preference

Update settlement preference

```
curl --request POST \
  --url https://api.exchange.coinbase.com/users/{user_id}/settlement-preferences \
  --header 'Content-Type: application/json' \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>' \
  --data '
{
  "settlement_preference": "<string>",
  "user_id": "self"
}
'
```