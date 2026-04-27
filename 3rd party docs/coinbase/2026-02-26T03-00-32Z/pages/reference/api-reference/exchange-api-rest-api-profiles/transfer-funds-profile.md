# transfer funds profile

Transfer funds between profiles

```
curl --request POST \
  --url https://api.exchange.coinbase.com/profiles/transfer \
  --header 'Content-Type: application/json' \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>' \
  --data '
{
  "from": "e543fd44-cbcd-4144-a2d9-6d81f42e2093",
  "to": "00921972-6b04-4daa-9458-3a38dd4924f6",
  "amount": "12.345",
  "currency": "BTC"
}
'
```