# create new locates

```
curl --request POST \
  --url https://api.prime.coinbase.com/v1/portfolios/{portfolio_id}/locates \
  --header 'Content-Type: application/json' \
  --data '
{
  "symbol": "BTC",
  "amount": "100",
  "conversion_date": "2023-11-01T00:00:00.000Z",
  "locate_date": "2023-11-01T00:00:00.000Z"
}
'
```