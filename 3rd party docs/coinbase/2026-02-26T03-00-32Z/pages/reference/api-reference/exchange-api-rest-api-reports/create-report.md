# create report

Reports provide batches of historic information about your profile in various human and machine readable forms.

## API Key Permissions

This endpoint requires either the “view” or “trade” permission.

## Expired reports

Reports are only available for download for a few days after being created. Once a report expires, the report is no longer available for download and is deleted.

## Balance Reports

Balance statements represent historical or current point-in-time “snapshots” in native and fiat converted units. Balance statement reports:

-   Can be generated for a **specific portfolio (API and [UI](https://exchange.coinbase.com/profile/statements)) or all portfolios (UI only)**. In the UI, all portfolios can be **grouped by portfolio or asset** (with balances totaled across portfolios).
-   Include **balances for crypto and fiat assets** supported on the Exchange (USD, EUR, GBP).
-   Include balances in both **native units** (e.g., 1 BTC) **as well as fiat-converted units assets** (e.g., $20000 USD worth of BTC) where price data is available.
-   Are generated for **all assets (crypto and fiat) in the user’s account** as of the requested timestamp. They cannot be generated for a specific asset (as is possible with the account and fill reports).

### Timestamps

-   **Range:** Timestamps are UTC-exclusive. For example, to generate a balance as of December 31st, 2022 EOD UTC (11:59:59 PM UTC), input January 1st, 2023 12:00:00 AM UTC.
-   **Granularity**: The API is the most granular and lets you specify a timestamp to the very second. The UI lets you specify the day and hour.

### Fiat Conversion

-   **For fiat balances** (USD, EUR, GBP), the conversion price is 1:1 and is reported in that specific fiat currency. EUR/GBP is not converted to USD balance.
-   **For crypto balances** the conversion price is the volume weighted average of closing prices in USD (when available). It is calculated by fetching 1 hour [candles](https://developer.chrome.com/api-reference/exchange-api/rest-api/products/get-product-candles) between \[t-24 hours to t\] and taking a volume weighted average of the closing price of the candles (when available).
    -   Requested timestamp = t; Start range = t - 24 hours; End range = t
    -   Candles may not be available; e.g., delisted assets may not have candles at the requested timestamp.
    -   If a USD pair is not listed for trading at the requested timestamp, fiat conversion is not possible.

### Request Details

The Balance Report API:

-   Leverages the existing `/reports` endpoint.
-   Adds a new report type of `balance`.
-   Adds a `balance` object to the request with `datetime` (and `group_by_portfolio_id` for the UI only).
-   Keeps the same response schema (with the possibility that `"type"="balance"`).

*Example of Balance Report API Request*

```
// Create balance statement for the portfolio tied to the API key
{
  "balance": {
    "datetime": "2022-02-25T05:00:00.000Z"
  },
  "email": "user1@example.com",
  "format": "csv",
  "type": "balance"
}

```

#### Authorizations

#### Body

Type of report to generate. Possible values: \[`account`, `balance`, `fills`, `otc-fills`, `rfq-fills`, `tax-invoice`, `1099k-transaction-history`\]

required for `1099k-transaction-history`\-type reports

Available options

:

`pdf`,

`csv`

Email to send generated report notification to

If this field is specified, it must be the `profile_id` that is linked to the API key.

#### Response

status

enum<string>

default:pending

required

Available options

:

`pending`