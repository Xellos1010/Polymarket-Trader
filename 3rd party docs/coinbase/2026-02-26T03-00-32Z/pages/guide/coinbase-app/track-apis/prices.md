# prices

## Table of Endpoints

Name

Method

Endpoint

Scope

[Get Buy Price](#get-buy-price)

GET

`/v2/prices/:currency_pair/buy`

N/A

[Get Sell Price](#get-sell-price)

GET

`/v2/prices/:currency_pair/sell`

N/A

[Get Spot Price](#get-spot-price)

GET

`/v2/prices/:currency_pair/spot`

N/A

## Get Buy Price

Get the total price to buy one bitcoin or ether. Note that exchange rates fluctuates so the price is only correct for seconds at the time. This buy price includes standard Coinbase fee (1%) but excludes any other fees including bank fees. If you need more accurate price estimate for a specific payment method or amount, see buy bitcoin endpoint and `quote: true` option. **This endpoint doesn’t require authentication.**

### HTTP Request

`GET https://api.coinbase.com/v2/prices/:currency_pair/buy`

### Scopes

-   *No permission required*

### Examples

#### Request

#### Response

```
{
  "data": {
    "amount": "1020.25",
    "currency": "USD"
  }
}

```

## Get Sell Price

Get the total price to sell one bitcoin or ether. Note that exchange rates fluctuates so the price is only correct for seconds at the time. This sell price includes standard Coinbase fee (1%) but excludes any other fees including bank fees. If you need more accurate price estimate for a specific payment method or amount, see sell bitcoin endpoint and `quote: true` option. **This endpoint doesn’t require authentication.**

### HTTP Request

`GET https://api.coinbase.com/v2/prices/:currency_pair/sell`

### Scopes

-   *No permission required*

### Examples

#### Request

#### Response

```
{
  "data": {
    "amount": "1010.25",
    "currency": "USD"
  }
}

```

## Get Spot Price

Get the current market price for bitcoin. This is usually somewhere in between the buy and sell price. Note that exchange rates fluctuates so the price is only correct for seconds at the time. You can also get historic prices with `date` parameter. **This endpoint doesn’t require authentication.**

### HTTP Request

`GET https://api.coinbase.com/v2/prices/:currency_pair/spot`

### Scopes

-   *No permission required*

### Arguments

Parameter

Type

Required

Description

date

string

Optional

For historic spot price, use format `YYYY-MM-DD` (UTC)

### Examples

#### Request

#### Response

```
{
  "data": {
    "amount": "1015.00",
    "currency": "USD"
  }
}

```