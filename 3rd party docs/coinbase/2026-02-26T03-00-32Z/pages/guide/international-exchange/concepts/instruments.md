# instruments

Coinbase International Exchange (INTX) supports trading across multiple instrument types, including perpetual futures and spot markets. The Instruments API provides access to instrument details, market data, funding rates, and historical trading volumes.

## Key Terminology

Before diving into the APIs, it’s important to understand how INTX distinguishes between core concepts:

Term

Definition

Example

**Asset**

A cryptocurrency that can be held, deposited, or withdrawn. Assets represent your account balances.

BTC, ETH, USDC

**Instrument**

A tradable product that defines what you’re buying or selling. Instruments can be perpetual futures or spot pairs.

BTC-PERP, ETH-USDC

**Order**

A request to buy or sell an instrument at a specified price and quantity. Orders are the parent record that fills are executed against.

Limit order to buy 0.5 BTC-PERP at $50,000

**Fill**

A completed trade execution recorded against an order. When an order matches, one or more fills are created with the executed price and quantity.

Fill of 0.5 BTC-PERP at $50,005 on order `14thr7eg-1-1`

**Position**

An open exposure in an instrument resulting from executed trades. Positions have a direction (long/short) and unrealized PnL.

Long 1.5 BTC-PERP

**Key distinctions:**

-   **Asset vs Instrument**: Assets are held (e.g., 10 USDC in a balance). Instruments are traded (e.g., BTC-PERP). An asset transfer moves cryptocurrency between portfolios. A position transfer moves an open derivatives position.
-   **Order vs Fill**: An order is the parent record representing a trade request. Fills are the individual executions recorded against that order. One order can produce multiple fills if matched in parts.
-   **Balance vs Position**: A balance reflects how much of an asset is held. A position reflects exposure to an instrument’s price movement.

## Instrument Types

INTX supports two primary instrument types:

-   **Perpetual (`PERP`)**: Perpetual futures contracts that never expire. These instruments track the underlying asset price and use a funding rate mechanism to keep prices aligned with spot markets.
-   **Spot (`SPOT`)**: Direct asset-to-asset trading pairs for immediate settlement.

## Listing Instruments

Use [List Instruments](https://developer.chrome.com/api-reference/international-exchange-api/rest-api/instruments/list-instruments) to retrieve all available trading instruments. This endpoint returns instrument specifications including trading states, modes, and configuration details.

```
from intx_sdk import IntxServicesClient
from intx_sdk.services.instruments import ListInstrumentsRequest
client = IntxServicesClient.from_env()
request = ListInstrumentsRequest()
response = client.instruments.list_instruments(request)
print(response)

```

To learn more about this SDK, please visit the [INTX Python SDK](https://github.com/coinbase-samples/intx-sdk-py).

## Getting Instrument Details

Retrieve detailed specifications for a specific instrument using [Get Instrument Details](https://developer.chrome.com/api-reference/international-exchange-api/rest-api/instruments/get-instrument-details). This includes tick size, lot size, margin requirements, and other trading parameters.

```
from intx_sdk import IntxServicesClient
from intx_sdk.services.instruments import GetInstrumentDetailsRequest
client = IntxServicesClient.from_env()
request = GetInstrumentDetailsRequest(
    instrument="BTC-PERP"
)
response = client.instruments.get_instrument_details(request)
print(response)

```

## Instrument Quotes

Get real-time quote data for an instrument using [Get Quote Per Instrument](https://developer.chrome.com/api-reference/international-exchange-api/rest-api/instruments/get-quote-per-instrument). This returns the current best bid/ask prices and sizes.

```
from intx_sdk import IntxServicesClient
from intx_sdk.services.instruments import GetQuotePerInstrumentRequest
client = IntxServicesClient.from_env()
request = GetQuotePerInstrumentRequest(
    instrument="BTC-PERP"
)
response = client.instruments.get_quote_per_instrument(request)
print(response)

```

## Aggregated Candles

Retrieve OHLCV (Open, High, Low, Close, Volume) candlestick data using [Get Aggregated Candles](https://developer.chrome.com/api-reference/international-exchange-api/rest-api/instruments/get-aggegated-candles). This is useful for charting and technical analysis.

```
from intx_sdk import IntxServicesClient
from intx_sdk.services.instruments import GetAggregatedCandlesRequest
from intx_sdk.enums import Granularity
client = IntxServicesClient.from_env()
request = GetAggregatedCandlesRequest(
    instrument="BTC-PERP",
    granularity=Granularity.ONE_HOUR.value,
    start="2025-01-01T00:00:00Z"
)
response = client.instruments.get_aggregated_candles(request)
print(response)

```

## Historical Funding Rates

For perpetual instruments, retrieve historical funding rate data using [Get Historical Funding Rates](https://developer.chrome.com/api-reference/international-exchange-api/rest-api/instruments/get-historical-funding-rate). Funding rates are periodic payments exchanged between long and short positions to keep perpetual prices aligned with spot markets.

```
from intx_sdk import IntxServicesClient
from intx_sdk.services.instruments import GetHistoricalFundingRatesRequest
client = IntxServicesClient.from_env()
request = GetHistoricalFundingRatesRequest(
    instrument="BTC-PERP"
)
response = client.instruments.get_historical_funding_rates(request)
print(response)

```

## Daily Trading Volumes

Retrieve historical daily trading volume data using [Get Daily Trading Volumes](https://developer.chrome.com/api-reference/international-exchange-api/rest-api/instruments/get-daily-trading-volume). This provides insights into market activity and liquidity.

```
from intx_sdk import IntxServicesClient
from intx_sdk.services.instruments import GetDailyTradingVolumesRequest
client = IntxServicesClient.from_env()
request = GetDailyTradingVolumesRequest(
    instruments="BTC-PERP",
    time_from="2025-01-01T00:00:00Z"
)
response = client.instruments.get_daily_trading_volumes(request)
print(response)

```