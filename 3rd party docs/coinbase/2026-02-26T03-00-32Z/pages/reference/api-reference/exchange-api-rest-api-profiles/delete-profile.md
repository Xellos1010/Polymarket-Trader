# delete profile

```
curl --request PUT \
  --url https://api.exchange.coinbase.com/profiles/{profile_id}/deactivate \
  --header 'Content-Type: application/json' \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>' \
  --data '
{
  "profile_id": "<string>",
  "to": "<string>"
}
'
```