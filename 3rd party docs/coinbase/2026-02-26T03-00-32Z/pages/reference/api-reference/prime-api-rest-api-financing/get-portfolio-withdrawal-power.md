# get portfolio withdrawal power

Get Portfolio Withdrawal Power

```
curl --request GET \
  --url https://api.prime.coinbase.com/v1/portfolios/{portfolio_id}/withdrawal_power
```

```
{
  "withdrawal_power": {
    "symbol": "BTC",
    "amount": "2.84882377"
  }
}
```

Returns the nominal quantity of a given asset that can be withdrawn based on holdings and current portfolio equity.

GET

/

v1

/

portfolios

/

{portfolio\_id}

/

withdrawal\_power

Get Portfolio Withdrawal Power

```
curl --request GET \
  --url https://api.prime.coinbase.com/v1/portfolios/{portfolio_id}/withdrawal_power
```

```
{
  "withdrawal_power": {
    "symbol": "BTC",
    "amount": "2.84882377"
  }
}
```

### Supported Products

-   Portfolio Margin
-   Trade Finance

#### Path Parameters

The unique ID of the portfolio

#### Query Parameters

#### Response