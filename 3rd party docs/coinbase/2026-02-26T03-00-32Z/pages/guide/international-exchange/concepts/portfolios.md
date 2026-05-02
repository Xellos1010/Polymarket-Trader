# portfolios

Portfolios on Coinbase International Exchange (INTX) are the primary unit for managing trading activity, balances, positions, and risk. Each portfolio allows you to view and manage margin, collateral, and position details within a scoped context. Each INTX account can have a maximum of **20 portfolios**. Portfolios can be updated and patched after creation to modify their name and settings.

## Listing Portfolios

Use [List Portfolios](https://developer.chrome.com/api-reference/international-exchange-api/rest-api/portfolios/list-all-user-portfolios) to retrieve all portfolios accessible to your INTX account. This is typically the first call made when initializing your trading integration.

```
from intx_sdk import IntxServicesClient
from intx_sdk.services.portfolios import ListPortfoliosRequest
client = IntxServicesClient.from_env()
request = ListPortfoliosRequest()
response = client.portfolios.list_portfolios(request)
print(response)

```

To learn more about this SDK, please visit the [INTX Python SDK](https://github.com/coinbase-samples/intx-sdk-py).

## Getting Portfolio Details

Retrieve detailed information about a specific portfolio using [Get Portfolio](https://developer.chrome.com/api-reference/international-exchange-api/rest-api/portfolios/get-user-portfolio).

```
from intx_sdk import IntxServicesClient
from intx_sdk.services.portfolios import GetPortfolioRequest
client = IntxServicesClient.from_env()
request = GetPortfolioRequest(
    portfolio="PORTFOLIO_ID_HERE"
)
response = client.portfolios.get_portfolio(request)
print(response)

```

## Creating a Portfolio

Create new portfolios to segregate trading strategies, risk profiles, or operational purposes using [Create Portfolio](https://developer.chrome.com/api-reference/international-exchange-api/rest-api/portfolios/create-portfolio).

```
from intx_sdk import IntxServicesClient
from intx_sdk.services.portfolios import CreatePortfolioRequest
client = IntxServicesClient.from_env()
request = CreatePortfolioRequest(
    name="Algorithmic Trading"
)
response = client.portfolios.create_portfolio(request)
print(response)

```

## Portfolio Balances

An asset is a cryptocurrency that can be held, deposited, or withdrawn (e.g., BTC, ETH, USDC). An asset balance represents the quantity of that asset held within a portfolio. Retrieve asset balances across a portfolio using [List Portfolio Balances](https://developer.chrome.com/api-reference/international-exchange-api/rest-api/portfolios/list-portfolio-balances). This returns the available, held, and total balance for each asset.

```
from intx_sdk import IntxServicesClient
from intx_sdk.services.portfolios import ListPortfolioBalancesRequest
client = IntxServicesClient.from_env()
request = ListPortfolioBalancesRequest(
    portfolio="PORTFOLIO_ID_HERE"
)
response = client.portfolios.list_portfolio_balances(request)
print(response)

```

## Portfolio Positions

A position represents an open exposure in an instrument, either long or short, with an associated entry price and unrealized PnL. Track open positions using [List Portfolio Positions](https://developer.chrome.com/api-reference/international-exchange-api/rest-api/portfolios/list-portfolio-positions). This returns all active positions with entry prices, unrealized PnL, and margin usage.

```
from intx_sdk import IntxServicesClient
from intx_sdk.services.portfolios import ListPortfolioPositionsRequest
client = IntxServicesClient.from_env()
request = ListPortfolioPositionsRequest(
    portfolio="PORTFOLIO_ID_HERE"
)
response = client.portfolios.list_portfolio_positions(request)
print(response)

```

## Portfolio Fills

A fill is a completed trade execution representing part or all of an order being matched. Fills are generated for all instrument types—both perpetual futures and spot trades appear in the fill history. Retrieve trade execution history using [List Portfolio Fills](https://developer.chrome.com/api-reference/international-exchange-api/rest-api/portfolios/list-portfolio-fills). Each fill includes the instrument, price, size, side, and fee details.

```
from intx_sdk import IntxServicesClient
from intx_sdk.services.portfolios import ListPortfolioFillsRequest
client = IntxServicesClient.from_env()
request = ListPortfolioFillsRequest(
    portfolio="PORTFOLIO_ID_HERE"
)
response = client.portfolios.list_portfolio_fills(request)
print(response)

```

## Fund Transfers (Asset Transfers)

Transfer assets between portfolios using [Transfer Funds](https://developer.chrome.com/api-reference/international-exchange-api/rest-api/portfolios/transfer-funds). This moves cryptocurrency holdings (e.g., USDC, BTC) from one portfolio to another, enabling efficient capital allocation across different trading strategies. Note: This transfers *assets* (your held balances), not positions. To move open derivatives positions, see [Position Transfers](#position-transfers).

```
from intx_sdk import IntxServicesClient
from intx_sdk.services.portfolios import TransferFundsRequest
client = IntxServicesClient.from_env()
request = TransferFundsRequest(
    from_portfolio="source-portfolio-id",
    to_portfolio="destination-portfolio-id",
    asset="USDC",  # The asset to transfer (not an instrument)
    quantity="1000"
)
response = client.portfolios.transfer_funds(request)

```

## Position Transfers

Transfer open positions between portfolios using [Transfer Position](https://developer.chrome.com/api-reference/international-exchange-api/rest-api/portfolios/transfer-positions). This moves an open derivatives position (e.g., a BTC-PERP long) from one portfolio to another—useful for consolidating positions or rebalancing across strategies. Note: This transfers *positions* in an instrument, not asset balances. To move cryptocurrency holdings, see [Fund Transfers](#fund-transfers-asset-transfers).

```
from intx_sdk import IntxServicesClient
from intx_sdk.services.portfolios import TransferPositionRequest
client = IntxServicesClient.from_env()
request = TransferPositionRequest(
    from_portfolio="source-portfolio-id",
    to_portfolio="destination-portfolio-id",
    instrument="BTC-PERP",
    quantity="0.5"
)
response = client.portfolios.transfer_position(request)

```

## Margin Management

### Auto Margin

Enable or disable automatic margin management using [Enable/Disable Auto Margin](https://developer.chrome.com/api-reference/international-exchange-api/rest-api/portfolios/enable-and-disable-portfolio). When enabled, the system automatically manages margin levels.

```
from intx_sdk import IntxServicesClient
from intx_sdk.services.portfolios import EnableDisableAutoMarginRequest
client = IntxServicesClient.from_env()
request = EnableDisableAutoMarginRequest(
    portfolio="PORTFOLIO_ID_HERE",
    enabled=True
)
response = client.portfolios.enable_disable_auto_margin(request)

```

### Cross Collateral

Enable or disable cross-collateral functionality using [Enable/Disable Cross Collateral](https://developer.chrome.com/api-reference/international-exchange-api/rest-api/portfolios/enable-and-disable-cross). Cross-collateral allows using multiple assets as margin for positions.

```
from intx_sdk import IntxServicesClient
from intx_sdk.services.portfolios import EnableDisableCrossCollateralRequest
client = IntxServicesClient.from_env()
request = EnableDisableCrossCollateralRequest(
    portfolio="PORTFOLIO_ID_HERE",
    enabled=True
)
response = client.portfolios.enable_disable_cross_collateral(request)

```

### Margin Override

Set custom margin requirements using [Set Margin Override](https://developer.chrome.com/api-reference/international-exchange-api/rest-api/portfolios/set-profile-margin).

```
from intx_sdk import IntxServicesClient
from intx_sdk.services.portfolios import SetMarginOverrideRequest
client = IntxServicesClient.from_env()
request = SetMarginOverrideRequest(
    portfolio="PORTFOLIO_ID_HERE",
    margin_override="0.1"
)
response = client.portfolios.set_margin_override(request)

```

## Fee Rates

Retrieve the fee schedule for a portfolio using [List Portfolio Fee Rates](https://developer.chrome.com/api-reference/international-exchange-api/rest-api/portfolios/list-portfolio-fee-rates).

```
from intx_sdk import IntxServicesClient
from intx_sdk.services.feerates import ListFeeRatesRequest
client = IntxServicesClient.from_env()
request = ListFeeRatesRequest(
    portfolio="PORTFOLIO_ID_HERE"
)
response = client.feerates.list_fee_rates(request)
print(response)

```