# update firm position limits 1

Update Firm Position Limits

```
curl --request POST \
  --url https://api.exchange.fairx.net/rest/v2/firm-position-limits/{firm_uuid} \
  --header 'Content-Type: application/json' \
  --data '
{
  "long_daily_limit": 1000000,
  "short_daily_limit": 500000,
  "long_weekend_limit": 1000000,
  "short_weekend_limit": 500000,
  "long_real_limit": 1000000,
  "short_real_limit": 500000,
  "weekend_margin_multiplier": 2
}
'
```