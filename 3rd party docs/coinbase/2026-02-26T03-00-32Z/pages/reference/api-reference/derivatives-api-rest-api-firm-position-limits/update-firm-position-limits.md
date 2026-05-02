# update firm position limits

Update Firm Position Limits

```
curl --request POST \
  --url https://api.exchange.fairx.net/rest/firm-position-limits/{firm_uuid} \
  --header 'Content-Type: application/json' \
  --data '
{
  "long_limit": 1000000,
  "short_limit": 500000,
  "weekend_margin_multiplier": 2
}
'
```