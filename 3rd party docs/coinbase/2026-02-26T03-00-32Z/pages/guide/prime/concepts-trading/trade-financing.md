# trade financing

Trades on Coinbase Prime are prefunded, meaning sufficient funds must be available in the portfolio before placing an order. However, portfolios with a trade finance line enabled can trade with T+1 settlement, allowing orders to be placed before funds are fully settled.

## How Trade Financing Works

When a trade finance line is added to a portfolio, it provides access to credit that can be used for trading. The borrowed amount must always be repaid, but fees are calculated based on a high watermark method. Utilization is measured daily at midnight UTC, tracking the maximum outstanding balance within each 24-hour period. For example, borrowing 1 BTC, paying it down, and borrowing 1 BTC again within the same period results in fees calculated on 1 BTC. However, borrowing 1 BTC and then borrowing an additional 1 BTC before paying down results in fees calculated on 2 BTC. Fees are currently billed at a later date rather than deducted at trade time. The FIX implementation has been designed to support real-time fee administration in the future.

## Monitoring Credit and Buying Power

The financing endpoints provide visibility into available credit, buying power, and withdrawal capacity.

### Get Portfolio Credit Information

Use [Get Portfolio Credit Information](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/get-portfolio-credit-information) to retrieve credit line details for a portfolio.

```
from prime_sdk.client_services import PrimeServicesClient
from prime_sdk.services.financing import GetPortfolioCreditInformationRequest
client = PrimeServicesClient.from_env()
request = GetPortfolioCreditInformationRequest(
    portfolio_id="PORTFOLIO_ID_HERE"
)
response = client.financing.get_portfolio_credit_information(request)
print(response)

```

For more information, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

### Get Portfolio Buying Power

Use [Get Portfolio Buying Power](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/get-portfolio-buying-power) to determine how much of a given asset can be purchased based on available funds and credit.

```
from prime_sdk.client_services import PrimeServicesClient
from prime_sdk.services.financing import GetBuyingPowerRequest
client = PrimeServicesClient.from_env()
request = GetBuyingPowerRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    base_currency="BTC",
    quote_currency="USD"
)
response = client.financing.get_portfolio_buying_power(request)
print(response)

```

For more information, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

### Get Portfolio Withdrawal Power

Use [Get Portfolio Withdrawal Power](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/get-portfolio-withdrawal-power) to check how much of a specific asset can be withdrawn while maintaining required collateral levels. Assets purchased using trade financing are available for withdrawal as long as the value of prefunded assets in the portfolio exceeds the amount purchased on credit.

```
from prime_sdk.client_services import PrimeServicesClient
from prime_sdk.services.financing import GetPortfolioWithdrawalPowerRequest
client = PrimeServicesClient.from_env()
request = GetPortfolioWithdrawalPowerRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    symbol="BTC"
)
response = client.financing.get_portfolio_withdrawal_power(request)
print(response)

```

For more information, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).